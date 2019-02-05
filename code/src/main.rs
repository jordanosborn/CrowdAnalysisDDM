// use rayon::prelude::*;
use std::sync::mpsc;

use arrayfire as af;
// use arrayfire::print_gen;
// use flame as fl;
use native::*;

pub mod native;
pub mod utils;

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

fn main() {
    let (tx, rx) = mpsc::channel::<Option<opencv::Image>>();
    //let id = opencv::start_capture_safe("./videos/colors.mp4");
    let id = opencv::start_camera_capture_safe();
    //For some reason this set_backend code needs to come after the start capture?
    set_backend();
    let stream_thread = std::thread::spawn(move || {
        for _ in 1..5 {
            let frame = opencv::Image::get_frame(id);
            match frame {
                None => {
                    match tx.send(None) {
                        _ => {
                            break;
                        }
                    };
                }
                Some(value) => match tx.send(Some(value)) {
                    Ok(_) => {
                        continue;
                    }
                    Err(_) => {
                        println!("Failed to send frame!");
                    }
                },
            }
        }
    });
    let process_thread = std::thread::spawn(move || loop {
        match rx.recv() {
            Ok(value) => {
                if let Some(v) = value {
                    af::save_image_native("img.png".to_string(), &v.data);
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
    });

    stream_thread.join().unwrap();
    process_thread.join().unwrap();
    opencv::close_stream_safe(id);
    utils::print_times();
}
