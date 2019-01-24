use arrayfire as af;
use arrayfire::print_gen;
use flame as fl;
pub mod image;


fn main() {
    unsafe {
        println!("{:?}", image::add(1, 2));
    }
    fl::span_of("name", || {
        let num_rows: u64 = 50;
        let num_cols: u64 = 30;
        let dims = af::Dim4::new(&[num_rows, num_cols, 1, 1]);
        let engine = af::RandomEngine::new(af::RandomEngineType::MERSENNE_GP11213, None);
        let a: af::Array<f32> = af::random_uniform(dims, &engine);
        af::af_print!("Create a 5-by-3 matrix of random floats on the GPU", a);
    });
    let spans = fl::spans();
    println!("{:?}", spans);
    
}
