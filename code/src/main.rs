// use rayon::prelude::*;
use std::sync::mpsc;
use arrayfire as af;

use native::*;
use operations::Data;

pub mod native;
pub mod operations;
pub mod utils;

type RawType = u8;

fn set_backend() {
    let backends = af::get_available_backends();
    let cuda_available = backends
        .iter()
        .filter(|&x| *x == af::Backend::CUDA)
        .count();
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
    let (tx, rx) = mpsc::channel::<Option<opencv::GrayImage>>();
    let (stx, srx) = mpsc::channel::<Signal>();
    let args = std::env::args().collect::<Vec<String>>();
    let args_slice = args.as_slice();
    let id = match args_slice {
        [_] => None,
        [_, command, path] if command == "video" => {
            Some(opencv::start_capture_safe(path))
        }
        [_, command] if command == "camera" => {
            Some(opencv::start_camera_capture_safe())
        }
        _ => None,
    };

    if let Some(id) = id {
        let fps = opencv::fps(id);
        set_backend();
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

        let mut data = Data::new(fps, None);
        loop {
            match rx.recv() {
                Ok(value) => {
                    if let Some(v) = value {
                        let ft = af::
                        data.push(v.data)
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
            //TODO: processing after each new frame

        }

        match stx.send(Signal::KILL) {
            Ok(_) | Err(_) => {
                stream_thread.join().unwrap();
                opencv::close_stream_safe(id);
                utils::print_times();
            }
        };
    }
}
