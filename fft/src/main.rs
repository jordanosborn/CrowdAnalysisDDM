// use rayon::prelude::*;
use std::sync::mpsc;

// use arrayfire as af;
// use arrayfire::print_gen;
// use flame as fl;
use native::*;

pub mod native;
pub mod utils;

fn main() {
    let (tx, rx) = mpsc::channel::<Option<opencv::Mat>>();
    let id = opencv::start_camera_capture_safe();
    let stream_thread = std::thread::spawn(move || {
        for _ in 0..10 {
            let frame = opencv::get_frame_safe(id);
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
                    let q = opencv::Image::new(&v);
                    let img = q.to_rgb_array();
                    img.save("out.jpg").expect("filenotsaved");
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
}
