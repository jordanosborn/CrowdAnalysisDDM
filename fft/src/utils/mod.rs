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
