#![allow(dead_code)]
fn g(alpha: (f64, f64, f64), x: f64) -> f64 {
    let (a, b, c) = alpha;
    a * (1.0 - f64::exp(c * x)) + b
}

fn numerical_derivative(f: fn(f64) -> f64, x: f64, delta: f64) -> f64 {
    (f(x + delta) - f(x - delta)) / (2.0 * delta)
}

fn linspace(start: f64, stop: f64, n: usize) -> Vec<f64> {
    let step = (stop - start) / (n as f64);
    (0..=n).map(|i| {
        (i as f64) * step + start
    }).collect()
}

fn least_squares(g: fn(f64) -> f64, x: &Vec<f64>, f: &Vec<f64>) -> f64 {
    x.iter().zip(f.iter()).map(|(&x, &f)| {
        (g(x) - f).powf(2.0)
    }).sum()
}


fn main() {
    let tau = linspace(0.0, 80.0, 1000);
    let f: Vec<f64> = tau.iter().map(|&t| g((1.0, 0.0, -0.1), t)).collect();
    let a = linspace(-1.0, 4.0, 1000);
    let c = linspace(-1.0, 1.0, 1000);
    println!("{}", numerical_derivative(|x| x*x, 3.0, 0.0001));

}