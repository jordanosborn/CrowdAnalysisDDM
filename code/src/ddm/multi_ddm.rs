#![allow(unused_imports)]
use crate::native::opencv;
use crate::utils::save_csv;
use arrayfire as af;
use rayon::prelude::*;
use std::collections::VecDeque;
use std::sync::mpsc;

use super::common::*;
use crate::fft_shift;
use crate::operations;
use crate::operations::Data;
use crate::utils::get_closest_power;
use crate::{RawFtType, RawType};

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
