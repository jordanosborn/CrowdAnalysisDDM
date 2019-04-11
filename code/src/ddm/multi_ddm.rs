#![allow(unused_imports)]
use crate::native::opencv;
use crate::utils::save_csv;
use arrayfire as af;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::VecDeque;
use std::sync::mpsc;

use super::common::*;
use crate::fft_shift;
use crate::operations;
use crate::operations::Data;
use crate::utils::get_closest_power;
use crate::wait;
use crate::{RawFtType, RawType};

fn get_allowed_dimension(
    tiling_min: usize,
    tiling_max: usize,
    tiling_size_count: usize,
) -> Vec<usize> {
    let xf64 = tiling_max as f64;
    let power2 = f64::log2(xf64).ceil() as i64;
    let power3 = f64::log(xf64, 3.0f64).ceil() as i64;
    let power5 = f64::log(xf64, 5.0f64).ceil() as i64;
    let mut box_range = (0..=power2)
        .cartesian_product((0..=power3).cartesian_product(0..=power5))
        .map(|(a, (b, c))| {
            (2.0f64.powf(a as f64) * 3.0f64.powf(b as f64) * 5.0f64.powf(c as f64)) as usize
        })
        .filter(|&value| tiling_min <= value && value <= tiling_max)
        .collect::<Vec<usize>>();
    box_range.sort();

    box_range
}

//TODO: implement this!
#[allow(unused_variables, clippy::too_many_arguments)]
pub fn multi_ddm(
    id: Option<usize>,
    capacity: Option<usize>,
    annuli_spacing: Option<usize>,
    tiling_range: (Option<usize>, Option<usize>, Option<usize>),
    activity_threshold: Option<usize>,
    tile_step: Option<usize>,
    filename: Option<String>,
    output_dir: Option<String>,
) -> Option<IndexedData> {
    let (tx, rx) = mpsc::channel::<Option<af::Array<RawType>>>();
    let (stx, srx) = mpsc::channel::<Signal>();
    let (annuli_tx, annuli_rx) =
        mpsc::channel::<Vec<(crate::RawType, arrayfire::Array<crate::RawType>)>>();

    let mut odim: Option<i64> = None;

    let annuli_spacing = if let Some(v) = annuli_spacing { v } else { 1 };
    let mut data_out = None;

    if let Some(id) = id {
        let (width, height) = opencv::dimension(id);
        if width != height {
            println!("Only square videos are supported!");
            return None;
        }

        let (tiling_min, tiling_max, tiling_size_count) =
            if let (Some(min), Some(max), Some(number)) = tiling_range {
                if max >= min && number != 0 {
                    (min, if max <= width { max } else { width }, number)
                } else {
                    println!("Invalid tiling range selected!");
                    return None;
                }
            } else {
                println!("Invalid tiling range selected!");
                return None;
            };

        let output_dir = if let Some(v) = filename {
            v
        } else {
            String::from("camera")
        };
        println!("Analysis of {} stream started!", &output_dir);
        let fps = opencv::fps(id);
        let frame_count = opencv::frame_count(id);

        let capacity = if let Some(c) = capacity {
            if c < frame_count {
                c
            } else {
                frame_count
            }
        } else {
            frame_count
        };

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
                    if odim == None {
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
                    }
                }
            }
            if let Ok(Signal::KILL) = srx.try_recv() {
                break;
            }
        });

        let mut counter_t0 = 0;
        let mut data: Data<crate::RawType> = Data::new(fps, Some(capacity));
        let mut collected_all_frames = false;
        let box_range = get_allowed_dimension(tiling_min, tiling_max, tiling_size_count);

        //TODO: here

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
            if collected_all_frames {
                if let Some(a) = accumulator {
                    let accumulator = a
                        .par_iter()
                        .map(|x| x / (counter_t0 as crate::RawType))
                        .collect::<Vec<af::Array<RawType>>>();
                    let annuli = match annuli_rx.recv() {
                        Ok(v) => v,
                        Err(e) => {
                            panic!("Failed to receive annuli - {}!", e);
                        }
                    };
                }
                break;
            }

            if data.data.len() == capacity {
                //TODO: process them before cap
                println!("{:#?}", box_range);
                wait!();
                counter_t0 += 1;
                println!("Analysis of t0 = {} done!", counter_t0);
            }
        }
        println!("Analysis of {} stream complete!", &output_dir);
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
