use arrayfire as af;
use arrayfire::print_gen;

fn main() {
    let num_rows: u64 = 5;
    let num_cols: u64 = 3;
    let dims = af::Dim4::new(&[num_rows, num_cols, 1, 1]);
    let engine = af::RandomEngine::new(af::RandomEngineType::MERSENNE_GP11213, None);
    let a: af::Array<f64> = af::random_uniform(dims, &engine);
    af::af_print!("Create a 5-by-3 matrix of random floats on the GPU", a);
}
