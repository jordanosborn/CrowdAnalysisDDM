use crate::native::opencv;
pub struct DDMArgs {
    pub stream_id: Option<usize>,
    pub capacity: Option<usize>,
    pub annuli_spacing: Option<usize>,
    pub filename: Option<String>,
    pub output: Option<String>,
}

pub struct MultiDDMArgs {
    pub stream_id: Option<usize>,
    pub capacity: Option<usize>,
    pub annuli_spacing: Option<usize>,
    pub tiling_range: (Option<usize>, Option<usize>, Option<usize>),
    pub filename: Option<String>,
    pub output_dir: Option<String>,
}

#[allow(dead_code)]
pub enum What {
    DDM(DDMArgs),
    CameraDDM(DDMArgs),
    MultiDDM(MultiDDMArgs),
    CameraMultiDDM(MultiDDMArgs),
    PROCESS(Option<String>),
    RETRANSPOSE(String, String),
    OTHER,
}

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
        [_, command, capacity, path] if command == "video-ddm" => What::DDM(DDMArgs {
            stream_id: Some(opencv::start_capture_safe(path)),
            capacity: Some(capacity.parse::<usize>().unwrap()),
            annuli_spacing: None,
            filename: match std::path::Path::new(path).file_stem() {
                Some(s) => Some(String::from(s.to_str().unwrap())),
                None => None,
            },
            output: None,
        }),
        [_, command, capacity, path, output]
            if command == "video-ddm" && output.ends_with(".csv") =>
        {
            What::DDM(DDMArgs {
                stream_id: Some(opencv::start_capture_safe(path)),
                capacity: Some(capacity.parse::<usize>().unwrap()),
                annuli_spacing: None,
                filename: match std::path::Path::new(path).file_stem() {
                    Some(s) => Some(String::from(s.to_str().unwrap())),
                    None => None,
                },
                output: Some(output.to_string()),
            })
        }
        [_, command, capacity, annuli_spacing, path] if command == "video-ddm" => {
            What::DDM(DDMArgs {
                stream_id: Some(opencv::start_capture_safe(path)),
                capacity: Some(capacity.parse::<usize>().unwrap()),
                annuli_spacing: Some(annuli_spacing.parse::<usize>().unwrap()),
                filename: match std::path::Path::new(path).file_stem() {
                    Some(s) => Some(String::from(s.to_str().unwrap())),
                    None => None,
                },
                output: None,
            })
        }
        [_, command, capacity, annuli_spacing, path, output]
            if command == "video-ddm" && output.ends_with(".csv") =>
        {
            What::DDM(DDMArgs {
                stream_id: Some(opencv::start_capture_safe(path)),
                capacity: Some(capacity.parse::<usize>().unwrap()),
                annuli_spacing: Some(annuli_spacing.parse::<usize>().unwrap()),
                filename: match std::path::Path::new(path).file_stem() {
                    Some(s) => Some(String::from(s.to_str().unwrap())),
                    None => None,
                },
                output: Some(output.to_string()),
            })
        }
        [_, command, capacity, annuli_spacing, tiling_min, tiling_max, tiling_step, path, output_dir]
            if command == "video-multi-ddm" =>
        {
            What::MultiDDM(MultiDDMArgs {
                stream_id: Some(opencv::start_capture_safe(path)),
                capacity: Some(capacity.parse::<usize>().unwrap()),
                annuli_spacing: Some(annuli_spacing.parse::<usize>().unwrap()),
                tiling_range: (
                    tiling_min.parse().ok(),
                    tiling_max.parse().ok(),
                    tiling_step.parse().ok(),
                ),
                filename: match std::path::Path::new(path).file_stem() {
                    Some(s) => Some(String::from(s.to_str().unwrap())),
                    None => None,
                },
                output_dir: if !std::path::Path::new(output_dir).exists() {
                    Some(output_dir.to_owned())
                } else {
                    panic!("Output directory already exists!")
                },
            })
        }
        [_, command, capacity, annuli_spacing, tiling_min, tiling_max, tiling_step, path]
            if command == "video-multi-ddm" =>
        {
            What::MultiDDM(MultiDDMArgs {
                stream_id: Some(opencv::start_capture_safe(path)),
                capacity: Some(capacity.parse::<usize>().unwrap()),
                annuli_spacing: Some(annuli_spacing.parse::<usize>().unwrap()),
                tiling_range: (
                    tiling_min.parse().ok(),
                    tiling_max.parse().ok(),
                    tiling_step.parse().ok(),
                ),
                filename: match std::path::Path::new(path).file_stem() {
                    Some(s) => Some(String::from(s.to_str().unwrap())),
                    None => None,
                },
                output_dir: None,
            })
        }
        [_, command, capacity, annuli_spacing, tiling_min, tiling_max, tiling_step, output_dir]
            if command == "camera-multi-ddm" =>
        {
            What::MultiDDM(MultiDDMArgs {
                stream_id: Some(opencv::start_camera_capture_safe()),
                capacity: Some(capacity.parse::<usize>().unwrap()),
                annuli_spacing: Some(annuli_spacing.parse::<usize>().unwrap()),
                tiling_range: (
                    tiling_min.parse().ok(),
                    tiling_max.parse().ok(),
                    tiling_step.parse().ok(),
                ),
                filename: None,
                output_dir: if !std::path::Path::new(output_dir).exists() {
                    Some(output_dir.to_owned())
                } else {
                    panic!("Output directory already exists!")
                },
            })
        }
        [_, command, capacity, output] if command == "camera-ddm" => What::CameraDDM(DDMArgs {
            stream_id: Some(opencv::start_camera_capture_safe()),
            capacity: Some(capacity.parse::<usize>().unwrap()),
            annuli_spacing: None,
            filename: None,
            output: if !std::path::Path::new(output).exists() {
                Some(output.to_owned())
            } else {
                panic!("Output file already exists!")
            },
        }),
        _ => What::OTHER,
    }
}
