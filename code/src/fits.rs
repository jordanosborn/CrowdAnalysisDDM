
use crate::ddm::multi_ddm::MultiDdmData;
#[allow(unused_imports)]
use crate::utils::save_csv;

use mathpack;
use mathpack::functions::basic::sinc;
use std::collections::HashMap;

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Fit {
    Brownian,
    Ballistic,
    CustomUnimplemented,
    CustomImplemented(String),
}

pub fn allowed_fit_type(fit_to: &str) -> bool {
    let split: Vec<_> = fit_to.split_ascii_whitespace().collect();
    ["brownian-fit", "ballistic-fit", "custom-fit"]
        .iter()
        .any(|x| split.iter().any(|y| *y == *x))
}

pub fn map_fit_type(fit_to: &str) -> Vec<Fit> {
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

pub fn brownian(vars: &[f64], params: &[f64]) -> f64 {
    assert!(vars.len() == 1 && params.len() == 3);
    params[0] * (1.0 - f64::exp(-vars[0] / params[1])) + params[2]
}

pub fn ballistic(vars: &[f64], params: &[f64]) -> f64 {
    assert!(vars.len() == 1 && params.len() == 4);
    params[0] * (1.0 - sinc(params[1] * vars[0]) * f64::exp(-vars[0] / params[2])) + params[3]
}

pub type FitResults = HashMap<Fit, Vec<f64>>;
pub type FitErrors = HashMap<Fit, f64>;

//TODO: allow input of rust function string use serde to deserialize!
pub fn create_custom_implementations(fit_to: Option<Vec<Fit>>) -> Option<Vec<Fit>> {
    let fit_to = fit_to?;
    Some(
        fit_to
            .iter()
            .filter_map(|x| match x {
                Fit::CustomUnimplemented => {
                    print!("Input closure using rust syntax (press enter when complete): ");
                    let func: Result<String, _> = try_read!();
                    match func {
                        Ok(f) => {
                            //TODO: implement function, if can parse custom_implemented(func);
                            Some(Fit::CustomImplemented(f))
                        }
                        Err(e) => {
                            println!("{}", e);
                            None
                        }
                    }
                }
                f => Some(f.clone()),
            })
            .collect(),
    )
}

fn get_fits(
    fit_to: &[Fit],
    vars: &[Vec<f64>],
    intensity: &[f64],
    weights: Vec<f64>,
) -> (FitResults, FitErrors) {
    fit_to
        .iter()
        .filter_map(|x| {
            match x {
                Fit::Brownian => Some((Fit::Brownian, brownian as fn(&[f64], &[f64]) -> f64)),
                Fit::Ballistic => Some((Fit::Ballistic, ballistic as fn(&[f64], &[f64]) -> f64)),
                Fit::CustomUnimplemented => None,
                Fit::CustomImplemented(_) => None, //TODO: deserialize function string!
            }
        })
        .map(|(fit_name, fit_func)| {
            let bounds = match fit_name {
                Fit::Brownian => (
                    vec![std::f64::MIN, 0.000_001, std::f64::MIN],
                    vec![std::f64::MAX; 3],
                ),
                Fit::Ballistic => (
                    vec![std::f64::MIN, std::f64::MIN, 0.000_001, std::f64::MIN],
                    vec![std::f64::MAX; 4],
                ),
                Fit::CustomImplemented(_) | Fit::CustomUnimplemented => (vec![], vec![]),
            };
            let (fit, err) = mathpack::fitting::fit_weighted(
                &fit_func,
                vars,
                intensity,
                &weights,
                bounds,
                None,
                None,
                (None, None),
                None,
            );
            ((fit_name.clone(), fit), (fit_name, err))
        })
        .unzip()
}
#[allow(non_snake_case, clippy::type_complexity)]
pub fn fit_ddm_results(
    data: Option<MultiDdmData>,
    fit_to: Vec<Fit>,
    filename: Option<String>,
    output_dir: Option<String>,
) -> Option<HashMap<usize, Vec<(f64, FitResults, FitErrors)>>> {
    let data = data?;
    let ret = data
        .iter()
        .map(|(k, v)| {
            let (q_vec, tau_I_vec) = v;
            (
                *k,
                q_vec
                    .iter()
                    .cloned()
                    .zip(tau_I_vec.iter().cloned())
                    .map(|(q, tau_I)| {
                        let weights: Vec<_> = tau_I.iter().map(|t| 1.0 / f64::from(t.0)).collect();
                        let (tau, I): (Vec<_>, Vec<_>) = tau_I
                            .iter()
                            .map(|t| (vec![f64::from(t.0)], f64::from(t.1)))
                            .unzip();
                        let (fits, errs) = get_fits(&fit_to, &tau, &I, weights);
                        (f64::from(q), fits, errs)
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();
    //TODO: save data to folder, plot and results
    Some(ret)
}