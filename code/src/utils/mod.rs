use arrayfire as af;
use arrayfire::{Array, Window};
use flame as fl;
use gnuplot;
use itertools::Itertools;
use rayon::prelude::*;
use std::io::prelude::*;

use crate::opencv;

#[allow(unused_macros)]
#[macro_export]
macro_rules! wait {
    () => {
        loop {
            println!("Continue? y/n");
            let o: char = read!("{}");
            if o == 'y' {
                break;
            }
        }
    };
}

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

#[allow(dead_code)]
pub fn save_plots(folder_name: &str, data: Vec<Vec<(crate::RawType, crate::RawType)>>) {
    let output_dir = format!("results/{}", String::from(folder_name));
    if !std::path::Path::new(&output_dir).exists() {
        std::fs::create_dir(&output_dir).expect("Can't create output directory!");
    }
    println!("Saving to {}", output_dir);
    for (index, graph) in data.iter().enumerate() {
        let mut x = Vec::with_capacity(graph.len());
        let mut y = Vec::with_capacity(graph.len());
        graph.iter().for_each(|(q, i)| {
            x.push(q);
            y.push(i);
        });
        let mut fg = gnuplot::Figure::new();
        fg.axes2d().lines(
            x,
            y,
            &[
                gnuplot::Caption(&format!("Tau = {}.", index + 1)),
                gnuplot::Color("black"),
            ],
        );
        fg.echo_to_file(&format!("{}/index{}.gplt", output_dir, index + 1));
        println!("Gplt for index = {} saved!", index + 1);
    }
}

#[allow(dead_code)]
pub fn save_images(acc: &[af::Array<crate::RawType>], filename: String) {
    let size = acc.len().to_string().chars().count();
    println!("Saving images to results/{}", filename);
    acc.iter().enumerate().for_each(|(i, x)| {
        let it = (i + 1).to_string();
        let mut s = String::from("");
        for _ in 0..(size - it.chars().count()) {
            s.push('0');
        }
        s.push_str(&it);
        let a = s.chars().join(".");
        af::save_image(format!("results/{}/{}.png", filename, a), x);
    });
}

#[allow(dead_code)]
pub fn save_csv<T: std::fmt::Display>(
    arr: &[Vec<(T, T)>],
    output_file: &str,
) -> std::io::Result<()> {
    let mut file = std::fs::File::create(std::path::Path::new(output_file))?;
    for line in arr.iter() {
        for item in line.iter() {
            let s = format!("({} {}),", item.0, item.1);
            file.write_all(&s.as_bytes())?;
        }
        file.write_all(b"\n")?;
    }
    Ok(())
}
