use crate::fits::*;
use crate::native::opencv;
pub struct DDMArgs {
    pub stream_id: Option<usize>,
    pub capacity: Option<usize>,
    pub annuli_spacing: Option<usize>,
    pub filename: Option<String>,
    pub output: Option<String>,
    pub fit_to: Option<Vec<Fit>>,
}

pub struct MultiDDMArgs {
    pub stream_id: Option<usize>,
    pub capacity: Option<usize>,
    pub annuli_spacing: Option<usize>,
    pub tiling_range: (Option<usize>, Option<usize>, Option<usize>),
    pub tile_step: Option<usize>,
    pub filename: Option<String>,
    pub output_dir: Option<String>,
    pub fit_to: Option<Vec<Fit>>,
}

pub enum What {
    DDM(DDMArgs),
    CameraDDM(DDMArgs),
    MultiDDM(MultiDDMArgs),
    CameraMultiDDM(MultiDDMArgs),
    PROCESS(Option<String>),
    RETRANSPOSE(String, String),
    OTHER,
}

#[allow(clippy::cognitive_complexity)]
pub fn process_arguments(args: Vec<String>) -> What {
    let args_slice = args.as_slice();
    match args_slice {
        [_, command, path, output]
            if command == "retranspose"
                && std::path::Path::new(path).exists()
                && path.ends_with(".csv")
                && output.ends_with(".csv") =>
        {
            What::RETRANSPOSE(path.clone(), output.clone())
        }
        [_, command, path] if command == "video-ddm" => What::DDM(DDMArgs {
            stream_id: Some(opencv::start_capture_safe(path)),
            capacity: Some(80),
            annuli_spacing: Some(1),
            filename: match std::path::Path::new(path).file_stem() {
                Some(s) => Some(String::from(s.to_str().unwrap())),
                None => None,
            },
            output: None,
            fit_to: None,
        }),
        [_, command, capacity, path] if command == "video-ddm" => What::DDM(DDMArgs {
            stream_id: Some(opencv::start_capture_safe(path)),
            capacity: capacity.parse().ok(),
            annuli_spacing: None,
            filename: match std::path::Path::new(path).file_stem() {
                Some(s) => Some(String::from(s.to_str().unwrap())),
                None => None,
            },
            output: None,
            fit_to: None,
        }),
        [_, command, capacity, path, output] if command == "video-ddm" => What::DDM(DDMArgs {
            stream_id: Some(opencv::start_capture_safe(path)),
            capacity: capacity.parse().ok(),
            annuli_spacing: None,
            filename: match std::path::Path::new(path).file_stem() {
                Some(s) => Some(String::from(s.to_str().unwrap())),
                None => None,
            },
            output: Some(output.to_string()),
            fit_to: None,
        }),
        [_, command, capacity, annuli_spacing, path] if command == "video-ddm" => {
            What::DDM(DDMArgs {
                stream_id: Some(opencv::start_capture_safe(path)),
                capacity: capacity.parse().ok(),
                annuli_spacing: annuli_spacing.parse().ok(),
                filename: match std::path::Path::new(path).file_stem() {
                    Some(s) => Some(String::from(s.to_str().unwrap())),
                    None => None,
                },
                output: None,
                fit_to: None,
            })
        }
        [_, command, capacity, annuli_spacing, path, output] if command == "video-ddm" => {
            What::DDM(DDMArgs {
                stream_id: Some(opencv::start_capture_safe(path)),
                capacity: capacity.parse().ok(),
                annuli_spacing: annuli_spacing.parse().ok(),
                filename: match std::path::Path::new(path).file_stem() {
                    Some(s) => Some(String::from(s.to_str().unwrap())),
                    None => None,
                },
                output: Some(output.to_string()),
                fit_to: None,
            })
        }
        [_, command, path] if command == "video-multi-ddm" => What::MultiDDM(MultiDDMArgs {
            stream_id: Some(opencv::start_capture_safe(path)),
            capacity: Some(80),
            annuli_spacing: Some(1),
            tiling_range: (None, None, None),
            tile_step: None,
            filename: match std::path::Path::new(path).file_stem() {
                Some(s) => Some(String::from(s.to_str().unwrap())),
                None => None,
            },
            output_dir: None,
            fit_to: None,
        }),
        [_, command, capacity, annuli_spacing, path] if command == "video-multi-ddm" => {
            What::MultiDDM(MultiDDMArgs {
                stream_id: Some(opencv::start_capture_safe(path)),
                capacity: capacity.parse().ok(),
                annuli_spacing: annuli_spacing.parse().ok(),
                tiling_range: (None, None, None),
                tile_step: None,
                filename: match std::path::Path::new(path).file_stem() {
                    Some(s) => Some(String::from(s.to_str().unwrap())),
                    None => None,
                },
                output_dir: None,
                fit_to: None,
            })
        }
        [_, command, capacity, annuli_spacing, tiling_min, tiling_max, tiling_size_count, path, output_dir]
            if command == "video-multi-ddm" =>
        {
            What::MultiDDM(MultiDDMArgs {
                stream_id: Some(opencv::start_capture_safe(path)),
                capacity: capacity.parse().ok(),
                annuli_spacing: annuli_spacing.parse().ok(),
                tiling_range: (
                    tiling_min.parse().ok(),
                    tiling_max.parse().ok(),
                    tiling_size_count.parse().ok(),
                ),
                tile_step: None,
                filename: match std::path::Path::new(path).file_stem() {
                    Some(s) => Some(String::from(s.to_str().unwrap())),
                    None => None,
                },
                output_dir: if !std::path::Path::new(output_dir).exists() {
                    Some(output_dir.to_owned())
                } else {
                    panic!("Output directory already exists!")
                },
                fit_to: None,
            })
        }
        [_, command, capacity, annuli_spacing, tiling_min, tiling_max, path]
            if command == "video-multi-ddm" =>
        {
            What::MultiDDM(MultiDDMArgs {
                stream_id: Some(opencv::start_capture_safe(path)),
                capacity: capacity.parse().ok(),
                annuli_spacing: annuli_spacing.parse().ok(),
                tiling_range: (tiling_min.parse().ok(), tiling_max.parse().ok(), None),
                tile_step: None,
                filename: match std::path::Path::new(path).file_stem() {
                    Some(s) => Some(String::from(s.to_str().unwrap())),
                    None => None,
                },
                output_dir: None,
                fit_to: None,
            })
        }
        [_, command, capacity, annuli_spacing, tiling_min, tiling_max, tiling_size_count, path]
            if command == "video-multi-ddm" =>
        {
            What::MultiDDM(MultiDDMArgs {
                stream_id: Some(opencv::start_capture_safe(path)),
                capacity: capacity.parse().ok(),
                annuli_spacing: annuli_spacing.parse().ok(),
                tiling_range: (
                    tiling_min.parse().ok(),
                    tiling_max.parse().ok(),
                    tiling_size_count.parse().ok(),
                ),
                tile_step: None,
                filename: match std::path::Path::new(path).file_stem() {
                    Some(s) => Some(String::from(s.to_str().unwrap())),
                    None => None,
                },
                output_dir: None,
                fit_to: None,
            })
        }
        [_, command, capacity, annuli_spacing, tiling_min, tiling_max, tiling_size_count, tile_step, path]
            if command == "video-multi-ddm" =>
        {
            What::MultiDDM(MultiDDMArgs {
                stream_id: Some(opencv::start_capture_safe(path)),
                capacity: capacity.parse().ok(),
                annuli_spacing: annuli_spacing.parse().ok(),
                tiling_range: (
                    tiling_min.parse().ok(),
                    tiling_max.parse().ok(),
                    tiling_size_count.parse().ok(),
                ),
                tile_step: tile_step.parse().ok(),
                filename: match std::path::Path::new(path).file_stem() {
                    Some(s) => Some(String::from(s.to_str().unwrap())),
                    None => None,
                },
                output_dir: None,
                fit_to: None,
            })
        }
        [_, command, capacity, annuli_spacing, tiling_min, tiling_max, tiling_size_count, output_dir]
            if command == "camera-multi-ddm" =>
        {
            What::MultiDDM(MultiDDMArgs {
                stream_id: Some(opencv::start_camera_capture_safe()),
                capacity: capacity.parse().ok(),
                annuli_spacing: annuli_spacing.parse().ok(),
                tiling_range: (
                    tiling_min.parse().ok(),
                    tiling_max.parse().ok(),
                    tiling_size_count.parse().ok(),
                ),
                tile_step: None,
                filename: None,
                output_dir: if !std::path::Path::new(output_dir).exists() {
                    Some(output_dir.to_owned())
                } else {
                    panic!("Output directory already exists!")
                },
                fit_to: None,
            })
        }
        [_, command, capacity, output] if command == "camera-ddm" => What::CameraDDM(DDMArgs {
            stream_id: Some(opencv::start_camera_capture_safe()),
            capacity: capacity.parse().ok(),
            annuli_spacing: None,
            filename: None,
            output: if !std::path::Path::new(output).exists() {
                Some(output.to_owned())
            } else {
                panic!("Output file already exists!")
            },
            fit_to: None,
        }),
        //TODO: range based slice patterns - until then split at spaces to use multiple fits
        [_, command, capacity, annuli_spacing, path, output, fit_to]
            if command == "video-ddm" && allowed_fit_type(fit_to) =>
        {
            What::DDM(DDMArgs {
                stream_id: Some(opencv::start_capture_safe(path)),
                capacity: capacity.parse().ok(),
                annuli_spacing: annuli_spacing.parse().ok(),
                filename: match std::path::Path::new(path).file_stem() {
                    Some(s) => Some(String::from(s.to_str().unwrap())),
                    None => None,
                },
                output: Some(output.to_string()),
                fit_to: Some(map_fit_type(fit_to)),
            })
        }
        [_, command, capacity, annuli_spacing, tiling_min, tiling_max, tiling_size_count, output_dir, fit_to]
            if command == "camera-multi-ddm" && allowed_fit_type(fit_to) =>
        {
            What::MultiDDM(MultiDDMArgs {
                stream_id: Some(opencv::start_camera_capture_safe()),
                capacity: capacity.parse().ok(),
                annuli_spacing: annuli_spacing.parse().ok(),
                tiling_range: (
                    tiling_min.parse().ok(),
                    tiling_max.parse().ok(),
                    tiling_size_count.parse().ok(),
                ),
                tile_step: None,
                filename: None,
                output_dir: if !std::path::Path::new(output_dir).exists() {
                    Some(output_dir.to_owned())
                } else {
                    panic!("Output directory already exists!")
                },
                fit_to: Some(map_fit_type(fit_to)),
            })
        }
        [_, command, capacity, output, fit_to]
            if command == "camera-ddm" && allowed_fit_type(fit_to) =>
        {
            What::CameraDDM(DDMArgs {
                stream_id: Some(opencv::start_camera_capture_safe()),
                capacity: capacity.parse().ok(),
                annuli_spacing: None,
                filename: None,
                output: if !std::path::Path::new(output).exists() {
                    Some(output.to_owned())
                } else {
                    panic!("Output file already exists!")
                },
                fit_to: Some(map_fit_type(fit_to)),
            })
        }
        [_, command, capacity, annuli_spacing, tiling_min, tiling_max, tiling_size_count, path, output_dir, fit_to]
            if command == "video-multi-ddm" && allowed_fit_type(fit_to) =>
        {
            What::MultiDDM(MultiDDMArgs {
                stream_id: Some(opencv::start_capture_safe(path)),
                capacity: capacity.parse().ok(),
                annuli_spacing: annuli_spacing.parse().ok(),
                tiling_range: (
                    tiling_min.parse().ok(),
                    tiling_max.parse().ok(),
                    tiling_size_count.parse().ok(),
                ),
                tile_step: None,
                filename: match std::path::Path::new(path).file_stem() {
                    Some(s) => Some(String::from(s.to_str().unwrap())),
                    None => None,
                },
                output_dir: if !std::path::Path::new(output_dir).exists() {
                    Some(output_dir.to_owned())
                } else {
                    panic!("Output directory already exists!")
                },
                fit_to: Some(map_fit_type(fit_to)),
            })
        }
        _ => What::OTHER,
    }
}
