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

pub type Pixel = u32;

pub trait PixelInt {
    fn new_pixel(dat: [u8; 3]) -> Pixel;
    fn as_pixel(self) -> image::Rgb<u8>;
}
//TODO: add operations on pixels.
impl PixelInt for Pixel {
    #[inline]
    fn new_pixel(dat: [u8; 3]) -> Pixel {
        //Opencv gives a BGR image this converts to RGB
        ((dat[2] as u32) << 16) | ((dat[1] as u32) << 8) | (dat[0] as u32)
    }
    #[inline]
    fn as_pixel(self) -> image::Rgb<u8> {
        image::Rgb {
            data: [
                (self >> 16) as u8,
                ((self >> 8) & 0xff) as u8,
                (self & 0xff) as u8,
            ],
        }
    }
}
