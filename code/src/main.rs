// use rayon::prelude::*;
use std::sync::mpsc;

use arrayfire as af;
use arrayfire::*;
// use arrayfire::print_gen;
// use flame as fl;
use native::*;

pub mod native;
pub mod operations;
pub mod utils;

type RawType = u8;

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
    let (tx, rx) = mpsc::channel::<Option<opencv::GrayImage>>();
    let (stx, srx) = mpsc::channel::<Signal>();
    //let id = opencv::start_capture_safe("./videos/colors.mp4");
    let id = opencv::start_camera_capture_safe();
    //For some reason this set_backend code needs to come after the start capture?
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
        match srx.try_recv() {
            Ok(Signal::KILL) => {
                break;
            }
            Err(_) => {}
        }
    });

    //UI code must run in main thread on Mac
    let mut win = Window::new(512, 512, "Crowd Analysis".to_string());
    let mut prev = opencv::GrayImage::empty();
    let mut output: Array<u8>;
    //TODO: processing code goes in here in separate thread possibly 
    while !win.is_closed() {
        match rx.recv() {
            Ok(value) => {
                if let Some(v) = value {
                    output = operations::difference(&v.data, &prev.data);
                    prev = opencv::GrayImage::from(v.data);
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
        win.draw_image(&output, None);
        win.show();
    }

    stx.send(Signal::KILL).unwrap();
    stream_thread.join().unwrap();
    opencv::close_stream_safe(id);
    utils::print_times();
}
