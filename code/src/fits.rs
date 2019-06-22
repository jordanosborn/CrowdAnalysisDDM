use mathpack;
pub enum Fit<'a> {
    Brownian,
    Ballistic,
    Custom(mathpack::fitting::Function<'a>),
}

#[inline]
fn sinc(x : f64) -> f64 {
    f64::sin(x) / x
}

pub fn brownian(vars: &[f64], params: &[f64]) -> f64 {
    assert!(vars.len() == 1 && params.len() == 3);
    params[0] * (1.0 - f64::exp(-vars[0] * params[1])) + params[2]
}

pub fn ballistic(vars: &[f64], params: &[f64]) -> f64 {
    assert!(vars.len() == 1 && params.len() == 4);
    params[0] * (1.0 - sinc(params[1] * vars[0]) * f64::exp(-vars[0] * params[2])) + params[3]
}