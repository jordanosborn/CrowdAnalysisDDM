#![allow(unused_imports)]
use crate::native::opencv;
use crate::utils::save_csv;
use arrayfire as af;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::sync::mpsc;

use super::common::*;
use crate::fft_shift;
use crate::operations;
use crate::operations::{sub_array, Data};
use crate::utils::get_closest_power;
use crate::wait;
use crate::{RawFtType, RawType};

fn get_allowed_dimension(
    tiling_min: usize,
    tiling_max: usize,
    tiling_size_count: Option<usize>,
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
    if let Some(tiling_size_count) = tiling_size_count {
        let tiling_size_count = if tiling_size_count <= box_range.len() {
            tiling_size_count
        } else {
            box_range.len()
        };
        let mut new_vec = Vec::with_capacity(tiling_size_count);
        let length = box_range.len() as f64;
        for i in 0..(tiling_size_count - 1) {
            new_vec
                .push(box_range[((i as f64) * length / (tiling_size_count as f64)).ceil() as usize])
        }
        new_vec.push(box_range[box_range.len() - 1]);
        new_vec
    } else {
        box_range
    }
}

type MultiDdmData = HashMap<usize, Vec<Vec<(crate::RawType, crate::RawType)>>>;

//TODO: implement this!
#[allow(clippy::too_many_arguments, clippy::cyclomatic_complexity)]
pub fn multi_ddm(
    id: Option<usize>,
    capacity: Option<usize>,
    annuli_spacing: Option<usize>,
    tiling_range: (Option<usize>, Option<usize>, Option<usize>),
    _activity_threshold: Option<usize>,
    tile_step: Option<usize>,
    filename: Option<String>,
    output_dir: Option<String>,
) -> Option<MultiDdmData> {
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
        let dimension = width;

        let (tiling_min, tiling_max, tiling_size_count) =
            if let (Some(min), Some(max), Some(number)) = tiling_range {
                if max >= min && number != 0 {
                    (
                        min,
                        if max <= width { max } else { dimension },
                        Some(number),
                    )
                } else {
                    println!("Invalid tiling range selected!");
                    return None;
                }
            } else if let (Some(min), Some(max), None) = tiling_range {
                if max >= min {
                    (min, if max <= width { max } else { dimension }, None)
                } else {
                    println!("Invalid tiling range selected!");
                    return None;
                }
            } else if let (None, None, None) = tiling_range {
                ((dimension as f64).log2() as usize, dimension, None)
            } else {
                println!("Invalid tiling range selected!");
                return None;
            };
        #[allow(unused_variables)]
        let tile_step = if let Some(t) = tile_step { t } else { 1 };

        let filename = if let Some(v) = filename {
            v
        } else {
            String::from("camera")
        };
        let output_dir = if let Some(v) = output_dir {
            v
        } else {
            format!("results-multiDDM/{}", filename)
        };

        println!(
            "Analysis of {} stream started! Results will be saved in {}",
            &filename, &output_dir
        );
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
                    if odim.is_some() {
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
        let mut images: Data<crate::RawType> = Data::new(fps, Some(capacity));
        let mut collected_all_frames = false;
        let box_range = get_allowed_dimension(tiling_min, tiling_max, tiling_size_count);
        let indices_range: Vec<Vec<(usize, usize)>> = box_range
            .par_iter()
            .map(|box_size| {
                (0..=(dimension - box_size))
                    .step_by(*box_size) //tile_step)
                    .cartesian_product((0..=(dimension - box_size)).step_by(*box_size)) //tile_step))
                    .collect()
            })
            .collect();

        #[allow(clippy::type_complexity)]
        let mut accumulator: HashMap<
            usize,
            HashMap<(usize, usize), Option<VecDeque<af::Array<crate::RawType>>>>,
        > = HashMap::with_capacity(box_range.len());
        loop {
            match rx.recv() {
                Ok(value) => {
                    if let Some(v) = value {
                        images.push(v);
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
                //retrieve all annuli
                let annuli = match annuli_rx.recv() {
                    Ok(v) => v,
                    Err(e) => {
                        panic!("Failed to receive annuli - {}!", e);
                    }
                };
                //T0 and radial average
                // box_size[tau[I(q)]]
                let mut box_size_map = HashMap::with_capacity(accumulator.len());
                for (box_size, v) in accumulator.iter_mut() {
                    let resized_annuli: Vec<_> = annuli
                        .par_iter()
                        .filter_map(|(q, arr)| {
                            let resized_arr = operations::sub_array(
                                arr,
                                (
                                    (dimension - box_size) as u64 / 2,
                                    (dimension - box_size) as u64 / 2,
                                ),
                                (
                                    (dimension + box_size) as u64 / 2,
                                    (dimension + box_size) as u64 / 2,
                                ),
                            )?;
                            let sum = af::sum_all(&resized_arr).0 as crate::RawType;
                            if sum > 0.0 {
                                Some((*q, resized_arr))
                            } else {
                                None
                            }
                        })
                        .collect();
                    println!("Resized annuli for boxsize = {}", box_size);

                    //averaged over
                    //TODO: v is valid here
                    //TODO: GOES wrong in here
                    //Time average and radial averaging for each x y
                    let acc_vec = v
                        .par_iter()
                        .map(|(key, &x)| {
                            if let Some(x) = x {
                                let vec_x = x.iter().map(|x| x / counter_t0).collect::<Vec<_>>();
                                (
                                    key,
                                    Some(operations::radial_average(&vec_x, &resized_annuli)),
                                )
                            } else {
                                (key, None)
                            }
                        })
                        .collect::<Vec<_>>();
                    // for (i, a) in acc_vec.clone().unwrap().iter().enumerate() {
                    //     println!("index {}", i);
                    //     af::print(a);
                    // }

                    //Add to box size map and perform box averaging and radial averaging and start time averaging

                    //TODO: box averaging
                    // let average = acc_vec
                    //     .into_par_iter()
                    //     .map(|x| x / (v.len()) as crate::RawType)
                    //     .collect::<Vec<_>>();
                    //Inserting these print statements prevents crash somehow?
                    println!("Averaged arrays for constant box_size = {}", box_size);

                    for ((x, y), Some(radial_average)) in acc_vec.iter() {
                        let (val_transposed_index, val_transposed) =
                            operations::transpose_2d_array(&radial_average);
                        let _ = save_csv(
                            &val_transposed_index,
                            &val_transposed,
                            &output_dir,
                            &format!("data_boxsize_{}_x_{}_y_{}.csv", box_size, x, y),
                        );
                        println!("Saved csv for boxsize = {} x, y = {}, {}", box_size, x, y);

                        box_size_map.insert(*box_size, radial_average);
                    }
                    println!("Finished averaging for boxsize = {}", box_size);
                }
                //TODO: run, upload to db and analyse
                println!("Multi-DDM complete!");
                data_out = Some(box_size_map);
                break;
            }

            if images.data.len() == capacity {
                for (box_id, box_size) in box_range.iter().enumerate() {
                    let indices = &indices_range[box_id];
                    //Ft of Tiles for each of the collected images
                    let tiled_images: Vec<Vec<_>> = images
                        .data
                        .par_iter()
                        .map(|im| {
                            indices
                                .par_iter()
                                .map(|(x, y)| {
                                    operations::sub_array(
                                        &im,
                                        (*x as u64, *y as u64),
                                        ((*x + box_size) as u64, (*y + box_size) as u64),
                                    )
                                })
                                .filter(std::option::Option::is_some)
                                .map(std::option::Option::unwrap)
                                .map(|d| {
                                    fft_shift!(af::fft2(
                                        &d,
                                        1.0,
                                        *box_size as i64,
                                        *box_size as i64
                                    ))
                                })
                                .collect()
                        })
                        .collect();
                    let tiled_images_ddm: Vec<_> = operations::transpose(tiled_images)
                        .par_iter()
                        .zip(indices.into_par_iter())
                        .map(|(arr, (x, y))| {
                            let ddmed = ddm(None, arr);
                            //Box_size and x, y
                            (*x, *y, ddmed)
                        })
                        .collect();
                    for (x, y, acc) in tiled_images_ddm.iter() {
                        if let Some(v1) = accumulator.get_mut(box_size) {
                            if let Some(v2) = v1.get_mut(&(*x, *y)) {
                                *v2 = operations::add_deque(v2.to_owned(), acc.to_owned());
                            } else {
                                v1.insert((*x, *y), acc.to_owned());
                            }
                        } else {
                            let mut h = HashMap::new();
                            h.insert((*x, *y), acc.to_owned());
                            accumulator.insert(*box_size, h);
                        }
                    }
                    println!("Tiled all images for box size {}", box_size);
                }
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
