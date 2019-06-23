use crate::ddm::multi_ddm::MultiDdmData;
#[allow(unused_imports)]
use crate::utils::save_csv;
use rayon::prelude::*;

use mathpack;
use mathpack::functions::basic::sinc;
use std::collections::HashMap;
use std::io::Write;
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
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
    params[0] * (1.0 - f64::exp(-vars[0] * params[1])) + params[2]
}

pub fn ballistic(vars: &[f64], params: &[f64]) -> f64 {
    assert!(vars.len() == 1 && params.len() == 4);
    params[0] * (1.0 - sinc(params[1] * vars[0]) * f64::exp(-vars[0] * params[2])) + params[3]
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
    _weights: Vec<f64>,
) -> (FitResults, FitErrors) {
    fit_to
        .par_iter()
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
            let (fit, err) = mathpack::fitting::fit(
                &fit_func,
                vars,
                intensity,
                //&weights,
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
    output_dir: Option<String>,
) -> Option<HashMap<usize, Vec<(f64, FitResults, FitErrors)>>> {
    println!("Producing fits to data!");
    let data = data?;
    let ret = data
        .par_iter()
        .map(|(k, v)| {
            println!("Starting fit for box size {}", k);
            let (q_vec, tau_I_vec) = v;
            let progress = indicatif::ProgressBar::new(q_vec.len() as u64);
            let ret = (
                *k,
                q_vec
                    .par_iter()
                    .zip(tau_I_vec.par_iter())
                    .map(|(q, tau_I)| {
                        let weights: Vec<_> = tau_I.iter().map(|t| 1.0 / f64::from(t.0)).collect();
                        let (tau, I): (Vec<_>, Vec<_>) = tau_I
                            .iter()
                            .map(|t| (vec![f64::from(t.0)], f64::from(t.1)))
                            .unzip();
                        let (fits, errs) = get_fits(&fit_to, &tau, &I, weights);
                        //TODO: fits are terrible?
                        progress.inc(1);
                        (f64::from(*q), fits, errs)
                    })
                    .collect::<Vec<_>>(),
            );
            progress.finish();
            ret
        })
        .collect::<HashMap<_, _>>();
    println!("Completed fits, now saving results!");
    let csv_format = ret
        .par_iter()
        .map(|(box_size, data)| {
            data.par_iter()
                .map(|(q, fits, errs)| {
                    fits.keys()
                        .map(move |fit_type| {
                            let vals = &fits[fit_type];
                            let err = errs[fit_type];
                            let vals_string = vals.iter().map(f64::to_string).collect::<Vec<_>>();
                            let vals_string = vals_string.join(", ");
                            format!(
                                "{}, {}, {}, {}, {}",
                                box_size,
                                q,
                                match fit_type {
                                    Fit::Brownian => "a1 * (1 - exp(-t * a2)) + a3",
                                    Fit::Ballistic => "a1 * (1 - sinc(a2 * t) * exp(-t * a3)) + a4",
                                    Fit::CustomUnimplemented => "",
                                    Fit::CustomImplemented(s) => s,
                                },
                                err,
                                vals_string,
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let output = output_dir.unwrap_or_else(|| String::from("fit_data.csv"));
    if let Some(path) = std::path::Path::new(&output).parent() {
        if !path.is_dir() {
            std::fs::create_dir(&output).expect("Can't create output directory!");
        }
    }
    let file = std::fs::File::create(std::path::Path::new(&output));
    match file {
        Ok(mut file) => {
            let header = file.write(b"q, box_size, fit_type, err, parameters...\n");
            let r = file.write_all(csv_format.as_bytes());
            match (r, header) {
                (Ok(_), Ok(_)) => println!("Saved fit data to {}", output),
                (Err(e), _) | (_, Err(e)) => {
                    println!("{} - Could not write fit data to file {}", e, output)
                }
            }
        }
        Err(e) => println!("{} - Data could not be saved", e),
    }
    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_brownian() {
        let data = crate::utils::read_csv("out/data_boxsize_800.csv", true).unwrap();
        let q = (1..data.len()).map(|x| x as f32 + 0.5).collect::<Vec<_>>();
        let mut map = HashMap::new();
        map.insert(800_usize, (q, data));
        fit_ddm_results(
            Some(map),
            vec![Fit::Brownian],
            Some(String::from("test1.csv")),
        );
    }
}