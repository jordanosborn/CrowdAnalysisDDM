use arrayfire as af;

use arrayfire::print_gen;
use flame as fl;
use rayon::prelude::*;
use native::opencv::CString;

pub mod native;

fn times(spans: &[fl::Span]) -> Vec<(&str, f64)> {
    spans
        .par_iter()
        .map(|x| (x.name.as_ref(), (x.delta as f64) / (10.0f64).powi(6)))
        .collect::<Vec<(&str, f64)>>()
}

fn main() {
    fl::span_of("name", || {
        af::set_backend(af::Backend::OPENCL);
        let num_rows: u64 = 5;
        let num_cols: u64 = 3;
        let dims = af::Dim4::new(&[num_rows, num_cols, 1, 1]);
        let engine = af::RandomEngine::new(af::RandomEngineType::MERSENNE_GP11213, None);
        let a: af::Array<f32> = af::random_uniform(dims, &engine);
        af::af_print!("Create a 5-by-3 matrix of random floats on the GPU", a);
    });
    let spans = fl::spans();
    let ti = times(&spans);
    ti.iter().for_each(|(x, y)| {
        println!("{} {}ms", x, y);
    });
    let id = native::opencv::start_camera_capture_safe();
    let mut i = 0;
    loop {
        let frame = native::opencv::get_frame_safe(id);
        if i == 20 {
            unsafe {
                native::opencv::write("/Users/jordan/Code/MastersProject/fft/homout.png".c_string().as_ptr(), frame.inner);
            }
            // let arr = af::Array::new(frame.data(), af::Dim4::new(&[frame.rows as u64, frame.cols as u64, 1, 1]));
            // af::save_image_native(String::from("test.png"), &arr);
            break;
        }
        i += 1;
    }
}
