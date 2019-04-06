use crate::native::opencv;
use crate::utils::save_csv;
use arrayfire as af;
use rayon::prelude::*;
use std::collections::VecDeque;
use std::sync::mpsc;

use crate::fft_shift;
use crate::operations;
use crate::operations::Data;
use crate::utils::get_closest_power;
use crate::{RawFtType, RawType};

fn ddm(
    accumulator: Option<VecDeque<arrayfire::Array<crate::RawType>>>,
    data: &VecDeque<arrayfire::Array<crate::RawFtType>>,
) -> Option<VecDeque<arrayfire::Array<crate::RawType>>> {
    match accumulator {
        Some(acc) => {
            let mut data_slice = data.clone();
            let ft0 = data_slice.pop_front().unwrap();
            Some(
                data_slice
                    .par_iter()
                    .zip(acc.par_iter())
                    .map(|(i, a)| {
                        //TODO: WTF why does this work when loc is added below???! panics at t0 = 47 ??????
                        //This works on mac mini
                        arrayfire::imin_all(a);
                        a + operations::difference(i, &ft0)
                    })
                    .collect::<VecDeque<arrayfire::Array<crate::RawType>>>(),
            )
        }
        None => {
            let mut data_slice = data.clone();
            let ft0 = data_slice.pop_front().unwrap();
            Some(
                data_slice
                    .par_iter()
                    .enumerate()
                    .map(|(_, x)| operations::difference(x, &ft0))
                    .collect::<VecDeque<arrayfire::Array<crate::RawType>>>(),
            )
        }
    }
}

enum Signal {
    KILL,
}

type IndexedData = (
    Vec<crate::RawType>,
    Vec<Vec<(crate::RawType, crate::RawType)>>,
);

pub fn single_ddm(
    id: Option<usize>,
    capacity: Option<usize>,
    annuli_spacing: Option<usize>,
    filename: Option<String>,
    output: Option<String>,
) -> Option<IndexedData> {
    let (tx, rx) = mpsc::channel::<Option<af::Array<RawFtType>>>();
    let (stx, srx) = mpsc::channel::<Signal>();
    let (annuli_tx, annuli_rx) =
        mpsc::channel::<Vec<(crate::RawType, arrayfire::Array<crate::RawType>)>>();

    let mut odim: Option<i64> = None;

    let annuli_spacing = if let Some(v) = annuli_spacing { v } else { 1 };
    let mut data_out = None;
    if let Some(id) = id {
        let mut output_dir;
        let output_name = if let Some(v) = output {
            output_dir = ".".to_string();
            v
        } else if let Some(v) = filename {
            output_dir = format!("results/{}", v);
            "radial_Avg.csv".to_string()
        } else {
            output_dir = ".".to_string();
            String::from("camera")
        };
        println!(
            "Analysis of {}/{} stream started!",
            &output_dir, &output_name
        );
        let fps = opencv::fps(id);
        let frame_count = opencv::frame_count(id);

        let capacity = if let Some(c) = capacity { c } else { fps };

        println!(
            "Video is about {} seconds long, containing {} frames!",
            (frame_count as f64) / (fps as f64),
            frame_count
        );
        let mut counter = 1u32;
        let stream_thread = std::thread::spawn(move || loop {
            let frame = opencv::GrayImage::get_frame(id);
            match frame {
                None => match tx.send(None) {
                    _ => {
                        break;
                    }
                },
                Some(value) => {
                    if let Some(dim) = odim {
                        let ft = fft_shift!(af::fft2(&value.data, 1.0, dim, dim));
                        match tx.send(Some(ft)) {
                            Ok(_) => {
                                println!("ft {} - complete!", counter);
                            }
                            Err(_) => {
                                println!("Failed to send frame!");
                            }
                        }
                        counter += 1;
                    } else {
                        let n = std::cmp::max(value.cols, value.rows);
                        odim = Some(get_closest_power(n as i64));
                        match annuli_tx
                            .send(operations::generate_annuli(n as u64, annuli_spacing as u64))
                        {
                            Ok(_) => println!("Generated annuli!"),
                            Err(e) => {
                                panic!("Failed to generate annuli - {}!", e);
                            }
                        }
                    }
                }
            }
            if let Ok(Signal::KILL) = srx.try_recv() {
                break;
            }
        });

        let mut counter_t0 = 0;
        let mut data: Data<crate::RawFtType> = Data::new(fps, Some(capacity));
        let mut collected_all_frames = false;

        let mut accumulator: Option<VecDeque<af::Array<RawType>>> = None;
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
                accumulator = ddm(accumulator, &data.data);
                counter_t0 += 1;
                println!("Analysis of t0 = {} done!", counter_t0);
            }

            if collected_all_frames {
                if let Some(a) = accumulator {
                    let accumulator = a
                        .par_iter()
                        .map(|x| x / (counter_t0 as f32))
                        .collect::<Vec<af::Array<RawType>>>();
                    let annuli = match annuli_rx.recv() {
                        Ok(v) => v,
                        Err(e) => {
                            panic!("Failed to receive annuli - {}!", e);
                        }
                    };
                    let radial_averaged = operations::radial_average(&accumulator, &annuli);
                    let radial_averaged_index = (1..=radial_averaged.len())
                        .map(|i| i as f32)
                        .collect::<Vec<f32>>();

                    let (r_avg_transposed_index, r_avg_transposed) =
                        operations::transpose_2d_array(&radial_averaged);

                    let _ = save_csv(
                        &radial_averaged_index,
                        &radial_averaged,
                        &output_dir,
                        &output_name,
                    );
                    let output_name_transposed =
                        output_name.clone().replace(".csv", "_transposed.csv");
                    let _ = save_csv(
                        &r_avg_transposed_index,
                        &r_avg_transposed,
                        &output_dir,
                        &output_name_transposed,
                    );

                    data_out = Some((r_avg_transposed_index, r_avg_transposed));
                }
                break;
            }
        }
        println!(
            "Analysis of {}/{} stream complete!",
            &output_dir, &output_name
        );
        match stx.send(Signal::KILL) {
            _ => {
                stream_thread.join().unwrap();
                opencv::close_stream_safe(id);
            }
        };
    } else {
        println!("Invalid arguments supplied!");
    }
    data_out
}

//TODO: implement this!
#[allow(unused_variables)]
pub fn multi_ddm(
    id: Option<usize>,
    capacity: Option<usize>,
    annuli_spacing: Option<usize>,
    tiling_range: Option<(usize, usize)>,
    filename: Option<String>,
    output_dir: Option<String>,
) {
    let (tx, rx) = mpsc::channel::<Option<af::Array<RawType>>>();
    let (stx, srx) = mpsc::channel::<Signal>();
    let (annuli_tx, annuli_rx) =
        mpsc::channel::<Vec<(crate::RawType, arrayfire::Array<crate::RawType>)>>();

    let mut odim: Option<i64> = None;

    let annuli_spacing = if let Some(v) = annuli_spacing { v } else { 1 };

    if let Some(id) = id {
        let output_dir = if let Some(v) = filename {
            v
        } else {
            String::from("camera")
        };
        println!("Analysis of {} stream started!", &output_dir);
        let fps = opencv::fps(id);
        let frame_count = opencv::frame_count(id);

        let capacity = if let Some(c) = capacity { c } else { fps };

        println!(
            "Video is about {} seconds long, containing {} frames!",
            (frame_count as f64) / (fps as f64),
            frame_count
        );
        let mut counter = 1u32;
        let stream_thread = std::thread::spawn(move || loop {
            let frame = opencv::GrayImage::get_frame(id);
            match frame {
                None => match tx.send(None) {
                    _ => {
                        break;
                    }
                },
                Some(value) => {
                    if let Some(dim) = odim {
                        match tx.send(Some(value.data)) {
                            Ok(_) => {
                                println!("Image capture {} - complete!", counter);
                            }
                            Err(_) => {
                                println!("Failed to send frame!");
                            }
                        }
                        counter += 1;
                    } else {
                        let n = std::cmp::max(value.cols, value.rows);
                        odim = Some(get_closest_power(n as i64));
                        match annuli_tx
                            .send(operations::generate_annuli(n as u64, annuli_spacing as u64))
                        {
                            Ok(_) => println!("Generated annuli!"),
                            Err(e) => {
                                panic!("Failed to generate annuli - {}!", e);
                            }
                        }
                    }
                }
            }
            if let Ok(Signal::KILL) = srx.try_recv() {
                break;
            }
        });
    }
}
