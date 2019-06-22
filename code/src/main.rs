#![deny(clippy::all)]

#[allow(unused_imports)]
#[macro_use]
extern crate text_io;

use arguments::{process_arguments, DDMArgs, MultiDDMArgs, What};
use arrayfire as af;

#[allow(unused_imports)]
use rayon::prelude::*;

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
                let _fit_data = fits::fit_ddm_results(res, fit_to, filename, out_dir);
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
                let _fit_data = fits::fit_ddm_results(res, fit_to, filename, out_dir);
            }
        }
        What::RETRANSPOSE(filename, output) => process::retranspose(&filename, &output),
        What::PROCESS(_) => {}
        What::OTHER => {
            println!("Invalid arguments supplied!");
        }
    }
}
