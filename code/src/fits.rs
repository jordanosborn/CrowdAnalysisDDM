use mathpack;
use crate::ddm;
//TODO: add more fit types here!

pub enum Fit<'a> {
    Brownian,
    Ballistic,
    CustomUnimplemented,
    CustomImplemented(mathpack::fitting::Function<'a>),
}

pub fn allowed_fit_type(fit_to: &str) -> bool {
    let split: Vec<_> = fit_to.split_ascii_whitespace().collect();
    ["brownian-fit", "ballistic-fit", "custom-fit"]
        .iter()
        .any(|x| split.iter().any(|y| *y == *x))
}

pub fn map_fit_type(fit_to: &str) -> Vec<Fit<'static>> {
    let split = fit_to.split_ascii_whitespace();
    split
        .filter_map(|s| match s {
            "brownian-fit" => Some(Fit::Brownian),
            "ballistic-fit" => Some(Fit::Ballistic),
            "custom-fit" => Some(Fit::CustomUnimplemented),
            _ => None,
        })
        .collect()
}

#[inline]
fn sinc(x: f64) -> f64 {
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

pub type FitResults = Vec<Vec<f64>>;

//TODO: implement these, save data, plot and results
pub fn fit_single_ddm_results(data: Option<ddm::common::IndexedData>, fit_to: Vec<Fit>, filename: Option<String>, output_dir: Option<String>) -> FitResults {

}

pub fn fit_ddm_results(data: Option<ddm::multi_ddm::MultiDdmData>, fit_to: Vec<Fit>, filename: Option<String>, output_dir: Option<String>) -> Vec<FitResults> {

}