#![deny(clippy::all)]

#[allow(unused_imports)]
#[macro_use]
extern crate text_io;
#[macro_use]
extern crate lazy_static;

use arguments::{process_arguments, DDMArgs, MultiDDMArgs, What};
use arrayfire as af;

#[allow(unused_imports)]
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
pub mod arguments;
pub mod ddm;
pub mod fits;
pub mod native;
pub mod operations;
#[macro_use]
pub mod utils;

pub mod process;

type RawType = f32;
#[inline]
pub fn raw_nan() -> RawType {
    std::f32::NAN
}
type RawFtType = num_complex::Complex32;

fn set_backend() {
    let backends = af::get_available_backends();
    let mut cuda_available = false;
    let mut opencl_available = false;
    backends.iter().for_each(|&x| {
        if x == af::Backend::CUDA {
            cuda_available = true;
        } else if x == af::Backend::OPENCL {
            opencl_available = true;
        }
    });
    if cuda_available {
        af::set_backend(af::Backend::CUDA);
    } else if opencl_available {
        af::set_backend(af::Backend::OPENCL);
    } else {
        af::set_backend(af::Backend::CPU);
    }
}

fn boxsize_from_string(s: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"boxsize_(\d+)").unwrap();
    }
    let cap = RE.captures(s).unwrap().get(1);
    let ret = match cap {
        Some(m) => m.as_str().to_string().parse::<usize>(),
        None => Ok(0),
    };

    match ret {
        Ok(v) => v,
        Err(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::boxsize_from_string;
    #[test]
    fn boxsize_from_string_test() {
        let res = boxsize_from_string("hello_boxsize_8243.csv");
        println!("{}", res);
        assert_eq!(res, 8243);
    }
}

fn main() {
    set_backend();
    let parsed_args = process_arguments(std::env::args().collect::<Vec<String>>());
    match parsed_args {
        What::DDM(DDMArgs {
            stream_id,
            capacity,
            annuli_spacing,
            filename,
            output,
            fit_to,
        })
        | What::CameraDDM(DDMArgs {
            stream_id,
            capacity,
            annuli_spacing,
            filename,
            output,
            fit_to,
        }) => {
            let fit_to = fits::create_custom_implementations(fit_to);
            let (out_dir, res) = ddm::single_ddm(
                stream_id,
                capacity,
                annuli_spacing,
                filename.clone(),
                output.clone(),
            );
            if let Some(fit_to) = fit_to {
                let _fit_data = fits::fit_ddm_results(
                    res,
                    fit_to,
                    Some(format!("{}/fit_data.csv", out_dir.unwrap())),
                );
            }
        }
        What::MultiDDM(MultiDDMArgs {
            stream_id,
            capacity,
            annuli_spacing,
            tiling_range,
            tile_step,
            filename,
            output_dir,
            fit_to,
        })
        | What::CameraMultiDDM(MultiDDMArgs {
            stream_id,
            capacity,
            annuli_spacing,
            tiling_range,
            tile_step,
            filename,
            output_dir,
            fit_to,
        }) => {
            let fit_to = fits::create_custom_implementations(fit_to);
            let (out_dir, res) = ddm::multi_ddm(
                stream_id,
                capacity,
                annuli_spacing,
                tiling_range,
                tile_step,
                filename.clone(),
                output_dir.clone(),
            );
            if let Some(fit_to) = fit_to {
                let _fit_data = fits::fit_ddm_results(
                    res,
                    fit_to,
                    Some(format!("{}/fit_data.csv", out_dir.unwrap())),
                );
            }
        }
        What::RETRANSPOSE(filename, output) => process::retranspose(&filename, &output),
        What::Fit(is_dir, path, fit_to) => {
            let fit_to = fits::map_fit_type(&fit_to);
            let mut map = HashMap::new();
            if is_dir {
                for entry in std::path::Path::new(&path)
                    .read_dir()
                    .unwrap_or_else(|_| panic!("Read of directory {} failed", path))
                {
                    let s = match entry {
                        Ok(v) => {
                            let path = format!("{:?}", v.path());
                            //Path quotes string
                            let s = path.replace("\"", "");
                            if s.find(".csv") == None {
                                break;
                            }
                            s
                        }
                        _ => break,
                    };
                    let box_size = boxsize_from_string(&s);
                    let data = crate::utils::read_csv(&s, true).unwrap();
                    let q = (1..data.len()).map(|x| x as f32 + 0.5).collect::<Vec<_>>();
                    map.insert(box_size, (q, data));
                }
            } else {
                let data = crate::utils::read_csv(&path, true).unwrap();
                let q = (1..data.len()).map(|x| x as f32 + 0.5).collect::<Vec<_>>();
                map.insert(boxsize_from_string(&path), (q, data));
            };
            let path = std::path::Path::new(&path);
            let _fit_results = match path.parent() {
                Some(p) => fits::fit_ddm_results(
                    Some(map),
                    fit_to,
                    Some(format!(
                        "{}/fit_data.csv",
                        p.to_str().unwrap_or_else(|| ".")
                    )),
                ),
                None => fits::fit_ddm_results(Some(map), fit_to, Some("fit_data.csv".to_string())),
            };
        }
        What::PROCESS(_) => {}
        What::OTHER => {
            println!("Invalid arguments supplied!");
        }
    }
}
