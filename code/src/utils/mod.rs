use arrayfire as af;
use arrayfire::{Array, Window};
use flame as fl;
use gnuplot;
use itertools::Itertools;
use rayon::prelude::*;
use std::io::prelude::*;

use super::native::opencv;

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

#[allow(unused_macros)]
#[macro_export]
macro_rules! fft_shift {
    ($item:expr) => {
        arrayfire::shift(
            &$item,
            &[
                ($item.dims()[0] / 2) as i32,
                ($item.dims()[1] / 2) as i32,
                1,
                1,
            ],
        );
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! fft_un_shift {
    ($item:expr) => {
        arrayfire::shift(
            &$item,
            &[
                ($item.dims()[0] * 2) as i32,
                ($item.dims()[1] * 2) as i32,
                1,
                1,
            ],
        );
    };
}

pub fn get_closest_power(x: i64) -> i64 {
    let xf64 = x as f64;
    let power2 = f64::log2(xf64).ceil() as i64;
    let power3 = f64::log(xf64, 3.0f64).ceil() as i64;
    let power5 = f64::log(xf64, 5.0f64).ceil() as i64;
    let minima = (0..=power2)
        .cartesian_product((0..=power3).cartesian_product(0..=power5))
        .map(|(a, (b, c))| {
            (2.0f64.powf(a as f64) * 3.0f64.powf(b as f64) * 5.0f64.powf(c as f64)) as i64
        })
        .filter(|&value| value >= x)
        .min();
    match minima {
        Some(n) => n,
        None => panic!("No suitable dimension!"),
    }
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
    index: &[T],
    arr: &[Vec<(T, T)>],
    output_dir: &str,
    output_filename: &str,
) -> std::io::Result<()> {
    if !std::path::Path::new(&output_dir).is_dir() {
        std::fs::create_dir(&output_dir).expect("Can't create output directory!");
    }
    let output = format!("{}/{}", output_dir, output_filename);
    let mut file = std::fs::File::create(std::path::Path::new(&output))?;
    for v in index.iter() {
        let s = format!("{},", v);
        file.write_all(&s.as_bytes())?;
    }
    file.write_all(b"\n")?;
    for line in arr.iter() {
        for item in line.iter() {
            let s = format!("({} {}),", item.0, item.1);
            file.write_all(&s.as_bytes())?;
        }
        file.write_all(b"\n")?;
    }
    Ok(())
}

#[allow(dead_code)]
pub fn read_csv(
    filename: &str,
    has_header: bool,
) -> Option<Vec<Vec<(crate::RawType, crate::RawType)>>> {
    match std::fs::File::open(filename) {
        Ok(f) => {
            let mut file = f;
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                let mut data = contents
                    .split('\n')
                    .filter(|&x| x != "")
                    .collect::<Vec<&str>>();
                if has_header {
                    data = data.drain(1..).collect();
                }
                Some(
                    data.iter()
                        .map(|&x| {
                            x.split(',')
                                .filter(|&x| x != "")
                                .map(|v| {
                                    let tmp = v.replace("(", "");
                                    let tmp = tmp.replace(")", "");
                                    let vector = tmp.split(' ').collect::<Vec<&str>>();
                                    match (vector[0].parse(), vector[1].parse()) {
                                        (Ok(n1), Ok(n2)) => (n1, n2),
                                        (_, _) => (crate::raw_nan(), crate::raw_nan()),
                                    }
                                })
                                .collect::<Vec<(crate::RawType, crate::RawType)>>()
                        })
                        .collect::<Vec<Vec<(crate::RawType, crate::RawType)>>>(),
                )
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
