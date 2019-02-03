use flame as fl;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    data: (u8, u8, u8),
}

#[derive(Debug, Copy, Clone)]
pub enum PIXEL {
    RGB(Rgb),
}

impl<'a> PIXEL {
    pub fn new_rgb(r: u8, g: u8, b: u8) -> PIXEL {
        PIXEL::RGB(Rgb { data: (r, g, b) })
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
