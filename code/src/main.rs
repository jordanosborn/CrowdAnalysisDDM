// use rayon::prelude::*;
use std::sync::mpsc;

use arrayfire as af;
#[allow(unused_imports)]
use gnuplot;
use itertools::Itertools;

use native::*;
use operations::Data;

pub mod native;
pub mod operations;
pub mod utils;

type RawType = f32;
type RawFtType = num_complex::Complex32;

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
    let (tx, rx) = mpsc::channel::<Option<opencv::GrayImage>>();
    let (stx, srx) = mpsc::channel::<Signal>();
    let args = std::env::args().collect::<Vec<String>>();
    let args_slice = args.as_slice();
    let id = match args_slice {
        [_] => None,
        [_, command, path] if command == "video" => Some(opencv::start_capture_safe(path)),
        [_, command] if command == "camera" => Some(opencv::start_camera_capture_safe()),
        _ => None,
    };

    if let Some(id) = id {
        println!("Analysis started!");
        let fps = opencv::fps(id);
        let stream_thread = std::thread::spawn(move || loop {
            let frame = opencv::GrayImage::get_frame(id);
            match frame {
                None => {
                    match tx.send(None) {
                        _ => {
                            break;
                        }
                    };
                }
                Some(value) => match tx.send(Some(value)) {
                    Ok(_) => {}
                    Err(_) => {
                        println!("Failed to send frame!");
                    }
                },
            }
            if let Ok(Signal::KILL) = srx.try_recv() {
                break;
            }
        });

        let mut data: Data<crate::RawFtType> = Data::new(fps, None);
        let mut counter = 0;
        loop {
            match rx.recv() {
                Ok(value) => {
                    if let Some(v) = value {
                        let ft = af::fft2(
                            &v.data,
                            1.0,
                            get_closest_power(v.cols as i64),
                            get_closest_power(v.rows as i64),
                        );
                        data.push(ft);
                        println!("ft {} - complete!", counter);
                        counter += 1;
                    } else {
                        break;
                    }
                }
                Err(e) => match std::sync::mpsc::TryRecvError::from(e) {
                    std::sync::mpsc::TryRecvError::Disconnected => {
                        break;
                    }
                    std::sync::mpsc::TryRecvError::Empty => {
                        continue;
                    }
                },
            }
            //TODO: processing after each new frame use rayon
        }

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
