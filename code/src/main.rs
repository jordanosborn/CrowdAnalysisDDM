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
    DDM,
    MultiDDM,
    PROCESS,
    OTHER,
}

fn process_arguments(
    args: Vec<String>,
) -> (
    Option<usize>,
    What,
    Option<usize>,
    Option<u64>,
    Option<String>,
) {
    let args_slice = args.as_slice();
    match args_slice {
        [_, command, path]
            if command == "process"
                && std::path::Path::new(path).exists()
                && path.ends_with(".csv") =>
        {
            (None, What::PROCESS, None, None, Some(path.clone()))
        }
        [_, command, capacity, path] if command == "video-ddm" => (
            Some(opencv::start_capture_safe(path)),
            What::DDM,
            Some(capacity.parse::<usize>().unwrap()),
            None,
            match std::path::Path::new(path).file_stem() {
                Some(s) => Some(String::from(s.to_str().unwrap())),
                None => None,
            },
        ),
        [_, command, capacity, annuli_spacing, path] if command == "video-ddm" => (
            Some(opencv::start_capture_safe(path)),
            What::DDM,
            Some(capacity.parse::<usize>().unwrap()),
            Some(annuli_spacing.parse::<u64>().unwrap()),
            match std::path::Path::new(path).file_stem() {
                Some(s) => Some(String::from(s.to_str().unwrap())),
                None => None,
            },
        ),
        [_, command, capacity] if command == "camera-ddm" => (
            Some(opencv::start_camera_capture_safe()),
            What::DDM,
            Some(capacity.parse::<usize>().unwrap()),
            None,
            None,
        ),
        _ => (None, What::OTHER, None, None, None),
    }
}

fn main() {
    set_backend();
    let (id, what, capacity, annuli_spacing, filename) =
        process_arguments(std::env::args().collect::<Vec<String>>());
    match what {
        What::DDM => ddm::single_ddm(id, capacity, annuli_spacing, filename),
        What::MultiDDM => ddm::multi_ddm(id, capacity, annuli_spacing, filename),
        What::PROCESS => process::retranspose(&filename.unwrap(), "output.csv"),
        What::OTHER => {
            println!("Invalid arguments supplied!");
        }
    }
}
