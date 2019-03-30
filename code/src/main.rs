#![deny(clippy::all)]

#[allow(unused_imports)]
#[macro_use]
extern crate text_io;

use arrayfire as af;
use native::opencv;

#[allow(unused_imports)]
use rayon::prelude::*;

pub mod ddm;
pub mod native;
pub mod operations;
#[macro_use]
pub mod utils;

pub mod process;

type RawType = f32;
type RawFtType = num_complex::Complex32;

fn set_backend() {
    let backends = af::get_available_backends();
    let cuda_available = backends.iter().filter(|&x| *x == af::Backend::CUDA).count();
    let opencl_available = backends
        .iter()
        .filter(|&x| *x == af::Backend::OPENCL)
        .count();
    if cuda_available == 1 {
        af::set_backend(af::Backend::CUDA);
    } else if opencl_available == 1 {
        af::set_backend(af::Backend::OPENCL);
    } else {
        af::set_backend(af::Backend::CPU);
    }
}

#[allow(dead_code)]
enum What {
    DDM(
        Option<usize>,
        Option<usize>,
        Option<usize>,
        Option<String>,
        Option<String>,
    ),
    MultiDDM(Option<usize>, Option<usize>, Option<usize>, Option<String>),
    PROCESS,
    RETRANSPOSE(String),
    OTHER,
}

fn process_arguments(args: Vec<String>) -> What {
    let args_slice = args.as_slice();
    match args_slice {
        [_, command, path]
            if command == "retranspose"
                && std::path::Path::new(path).exists()
                && path.ends_with(".csv") =>
        {
            What::RETRANSPOSE(path.clone())
        }
        [_, command, capacity, path] if command == "video-ddm" => What::DDM(
            Some(opencv::start_capture_safe(path)),
            Some(capacity.parse::<usize>().unwrap()),
            None,
            match std::path::Path::new(path).file_stem() {
                Some(s) => Some(String::from(s.to_str().unwrap())),
                None => None,
            },
            None,
        ),
        [_, command, capacity, path, output] if command == "video-ddm" => What::DDM(
            Some(opencv::start_capture_safe(path)),
            Some(capacity.parse::<usize>().unwrap()),
            None,
            match std::path::Path::new(path).file_stem() {
                Some(s) => Some(String::from(s.to_str().unwrap())),
                None => None,
            },
            Some(output.to_string()),
        ),
        [_, command, capacity, annuli_spacing, path, output] if command == "video-ddm" => {
            What::DDM(
                Some(opencv::start_capture_safe(path)),
                Some(capacity.parse::<usize>().unwrap()),
                Some(annuli_spacing.parse::<usize>().unwrap()),
                match std::path::Path::new(path).file_stem() {
                    Some(s) => Some(String::from(s.to_str().unwrap())),
                    None => None,
                },
                Some(output.to_string()),
            )
        }
        [_, command, capacity] if command == "camera-ddm" => What::DDM(
            Some(opencv::start_camera_capture_safe()),
            Some(capacity.parse::<usize>().unwrap()),
            None,
            None,
            None,
        ),
        _ => What::OTHER,
    }
}

fn main() {
    set_backend();
    let parsed_args = process_arguments(std::env::args().collect::<Vec<String>>());
    match parsed_args {
        What::DDM(id, capacity, annuli_spacing, filename, output) => {
            ddm::single_ddm(id, capacity, annuli_spacing, filename, output);
        }
        What::MultiDDM(id, capacity, annuli_spacing, filename) => {
            ddm::multi_ddm(id, capacity, annuli_spacing, filename);
        }
        What::RETRANSPOSE(filename) => process::retranspose(&filename, "output.csv"),
        What::PROCESS => {}
        What::OTHER => {
            println!("Invalid arguments supplied!");
        }
    }
}
