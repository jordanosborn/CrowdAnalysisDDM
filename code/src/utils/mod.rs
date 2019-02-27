use flame as fl;
use rayon::prelude::*;

pub fn times() -> Vec<(String, f64)> {
    let spans = fl::spans();
    spans
        .par_iter()
        .map(|x| {
            (
                String::from(x.name.as_ref()),
                (x.delta as f64) / (10.0f64).powi(6),
            )
        })
        .collect::<Vec<(String, f64)>>()
}

pub fn print_times() {
    let ti = times();
    ti.iter().for_each(|(x, y)| {
        println!("{} {}ms", x, y);
    });
}

use arrayfire::{Array, Window};
use crate::opencv;

#[allow(dead_code)]
fn show_frames(rx: &std::sync::mpsc::Receiver<Option<opencv::GrayImage>>) {
    let mut win = Window::new(512, 512, "Crowd Analysis".to_string());
    let mut output: Array<crate::RawType>;
    while !win.is_closed() {
        match rx.recv() {
            Ok(value) => {
                if let Some(v) = value {
                    output = v.data;
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
}
