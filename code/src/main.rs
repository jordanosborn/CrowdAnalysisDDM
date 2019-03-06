#[allow(unused_imports)]
#[macro_use]
extern crate text_io;

use std::collections::VecDeque;
use std::str::FromStr;
use std::sync::mpsc;

use arrayfire as af;
#[allow(unused_imports)]
use gnuplot;
use itertools::Itertools;
#[allow(unused_imports)]
use rayon::prelude::*;

use native::*;
use operations::Data;

pub mod ddm;
pub mod native;
pub mod operations;
pub mod utils;

type RawType = f32;
type RawFtType = num_complex::Complex32;

#[allow(unused_macros)]
macro_rules! print_wait {
    ($item:expr) => {
        af::print_gen("".to_string(), &$item, Some(2));
        let _: u32 = read!("{}");
    };
}

macro_rules! fft_shift {
    ($item:expr) => {
        arrayfire::shift(&$item, &[($item.dims()[0] / 2) as i32, ($item.dims()[1] / 2) as i32, 0, 0]);
    }
}

fn get_closest_power(x: i64) -> i64 {
    let xf64 = x as f64;
    let power2 = f64::log2(xf64).ceil() as i64;
    let power3 = f64::log(xf64, 3.0f64).ceil() as i64;
    let power5 = f64::log(xf64, 5.0f64).ceil() as i64;
    let minima = (0..=power2)
        .cartesian_product((0..=power3).cartesian_product(0..=power5))
        .map(|(a, (b, c))| {
            (2.0f64.powf(a as f64) * 3.0f64.powf(b as f64) * 5.0f64.powf(c as f64)) as i64
        })
        .filter(|&value| value >= x)
        .min();
    match minima {
        Some(n) => n,
        None => panic!("No suitable dimension!"),
    }
}

fn set_backend() {
    let backends = af::get_available_backends();
    let cuda_available = backends.iter().filter(|&x| *x == af::Backend::CUDA).count();
    let opencl_available = backends
        .iter()
        .filter(|&x| *x == af::Backend::OPENCL)
        .count();
    if cuda_available == 1 {
        af::set_backend(af::Backend::CUDA);
    } else if opencl_available == 1 {
        af::set_backend(af::Backend::OPENCL);
    } else {
        af::set_backend(af::Backend::CPU);
    }
}

enum Signal {
    KILL,
}

fn main() {
    set_backend();
    let (tx, rx) = mpsc::channel::<Option<af::Array<RawFtType>>>();
    let (stx, srx) = mpsc::channel::<Signal>();
    let args = std::env::args().collect::<Vec<String>>();
    let args_slice = args.as_slice();

    // Args processing should extract!
    let (id, average_over, filename) = match args_slice {
        [_, command, path] if command == "video" => (
            Some(opencv::start_capture_safe(path)),
            None,
            std::path::Path::new(path).file_stem(),
        ),
        [_, command, path, avg] if command == "video" => {
            if let Ok(avg) = usize::from_str(avg) {
                (
                    Some(opencv::start_capture_safe(path)),
                    Some(avg),
                    std::path::Path::new(path).file_name(),
                )
            } else {
                (None, None, std::path::Path::new(path).file_name())
            }
        }
        [_, command] if command == "camera" => {
            (Some(opencv::start_camera_capture_safe()), None, None)
        }
        _ => (None, None, None),
    };

    let mut odim0: Option<i64> = None;
    let mut odim1: Option<i64> = None;

    if let Some(id) = id {
        println!(
            "Analysis of {} started!",
            if let Some(filename) = filename {
                filename.to_str().unwrap()
            } else {
                "camera stream"
            }
        );
        let fps = opencv::fps(id);
        let frame_count = opencv::frame_count(id);
        println!(
            "Video is about {} seconds long, containing {} frames!",
            (frame_count as f64) / (fps as f64),
            frame_count
        );
        let mut counter = 1u32;
        let stream_thread = if let Some(average_over) = average_over {
            let mut frames_to_average: VecDeque<af::Array<RawType>> =
                VecDeque::with_capacity(average_over);
            std::thread::spawn(move || loop {
                let frame = opencv::GrayImage::get_frame(id);
                match frame {
                    None => {
                        match tx.send(None) {
                            _ => {
                                break;
                            }
                        };
                    }
                    Some(value) => {
                        if odim0 == None || odim1 == None {
                            odim0 = Some(get_closest_power(value.cols as i64));
                            odim1 = Some(get_closest_power(value.rows as i64));
                        }
                        frames_to_average.push_back(value.data);
                        if frames_to_average.len() == average_over + 1 {
                            frames_to_average.pop_front();
                            if let Some(value) = operations::mean_image(&frames_to_average) {
                                let ft = af::fft2(&value, 1.0, odim0.unwrap(), odim1.unwrap());
                                println!("ft {} - complete!", counter);
                                counter += 1;
                                match tx.send(Some(ft)) {
                                    Ok(_) => {}
                                    Err(_) => {
                                        println!("Failed to send frame!");
                                    }
                                }
                            }
                        };
                    }
                }
                if let Ok(Signal::KILL) = srx.try_recv() {
                    break;
                }
            })
        } else {
            std::thread::spawn(move || loop {
                let frame = opencv::GrayImage::get_frame(id);
                match frame {
                    None => {
                        match tx.send(None) {
                            _ => {
                                break;
                            }
                        };
                    }
                    Some(value) => {
                        if odim0 == None || odim1 == None {
                            odim0 = Some(get_closest_power(value.cols as i64));
                            odim1 = Some(get_closest_power(value.rows as i64));
                        }
                        let ft = fft_shift!(af::fft2(&value.data, 1.0, odim0.unwrap(), odim1.unwrap()));
                        match tx.send(Some(ft)) {
                            Ok(_) => {
                                println!("ft {} - complete!", counter);
                            }
                            Err(_) => {
                                println!("Failed to send frame!");
                            }
                        }
                        counter += 1;
                    }
                }
                if let Ok(Signal::KILL) = srx.try_recv() {
                    break;
                }
            })
        };

        let capacity = fps; //* 1;
        let output_dir = if let Some(filename) = filename {
            format!("results/{}", filename.to_str().unwrap_or(""))
        } else {
            "results".to_string()
        };
        if !std::path::Path::new(&output_dir).exists() {
            std::fs::create_dir(&output_dir).expect("Can't create output directory!");
        }

        let mut counter_t0 = 0;
        let mut data: Data<crate::RawFtType> = Data::new(fps, Some(capacity));
        let mut collected_all_frames = false;

        let mut acc: Option<VecDeque<af::Array<RawType>>> = None;
        loop {
            match rx.recv() {
                Ok(value) => {
                    if let Some(v) = value {
                        data.push(v);
                    }
                }
                Err(e) => match std::sync::mpsc::TryRecvError::from(e) {
                    std::sync::mpsc::TryRecvError::Disconnected => {
                        collected_all_frames = true;
                    }
                    std::sync::mpsc::TryRecvError::Empty => {
                        continue;
                    }
                },
            }

            if data.data.len() == capacity {
                if let Some(a) = acc {
                    acc = Some(ddm::ddm(a, &data.data));
                } else {
                    acc = Some(ddm::ddm_0(&data.data));
                };
                counter_t0 += 1;
                println!("Analysis of t0 = {} done!", counter_t0);
            }

            if collected_all_frames {
                if let Some(a) = acc {
                    let acc = a
                        .par_iter()
                        .map(|x| x / (counter_t0 as f32))
                        .collect::<VecDeque<af::Array<RawType>>>();
                    let acc = operations::radial_average(acc);
                    //TODO: Create some graphs after radial averaging! I vs q^2 for various tau
                    let size = acc.len().to_string().chars().count();
                    let filename  = filename.unwrap().to_str().unwrap();
                    println!("Saving images to results/{}", filename);
                    acc.iter().enumerate().for_each(|(i, x)| {
                        let it = (i+1).to_string();
                        let mut s = String::from("");
                        for _ in 0..(size-it.chars().count()) {
                            s.push('0');
                        }
                        s.push_str(&it);
                        let a = s.chars().join(".");
                        af::save_image_native(format!("results/{}/{}.png", filename, a), x);
                    })
                }
                break;
            }
        }
        println!(
            "Analysis of {} complete!",
            if let Some(filename) = filename {
                filename.to_str().unwrap()
            } else {
                "camera stream"
            }
        );
        match stx.send(Signal::KILL) {
            Ok(_) | Err(_) => {
                stream_thread.join().unwrap();
                opencv::close_stream_safe(id);
                utils::print_times();
            }
        };
    } else {
        println!("No arguments supplied!");
    }
}
