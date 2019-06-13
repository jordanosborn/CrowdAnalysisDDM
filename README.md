# Crowd Analysis using DDM

[![Build Status](https://travis-ci.com/jordanosborn/MastersProject.svg?token=2eJkyoJzDLeBMdiGDz2x&branch=master)](https://travis-ci.com/jordanosborn/MastersProject)
## Set Up
### Requirements
1. Ubuntu 18.04+ or macOS Mojave+
1. (Optional) NVIDIA Jetson TX2 - set up using Jetpack tool
1. Codecs installed for videos you wish to analyse
1. OpenCV4
1. ArrayFire 3.6
1. Cmake 3.11 +
1. [Rust](https://rustup.rs)
1. Clang
1. Python3.6+
1. [Poetry](https://github.com/sdispater/poetry)
#### Install on Ubuntu:
[Download](https://github.com/jordanosborn/CrowdAnalysisDDM/raw/master/install_ubuntu.sh) installer script.

    curl https://github.com/jordanosborn/CrowdAnalysisDDM/raw/master/install_ubuntu.sh > install_ubuntu.sh 
    chmod +x install_ubuntu.sh
    ./install_ubuntu.sh
#### Install on Mac:
TODO - follow README in [code folder](code/README.md).
### Usage - Command Line Arguments

Inside code directory replace {arg} with path/ numerical value (positive integers)

#### Cargo Options
    cargo run --release video-ddm {frame_buffer_capacity} {annuli_spacing} {video_path} {output_csv_path}
    cargo run --release camera-ddm {frame_buffer_capacity} {outup_csv_path}
    cargo run --release camera-multi-ddm {frame_buffer_capacity} {annuli_spacing} {tile_min_size} {tile_max_size} {number_tiles} {output_directory_path}
    cargo run --release video-multi-ddm {frame_buffer_capacity} {annuli_spacing} {tile_min_size} {tile_max_size} {number_tiles} {video_path} {output_directory_path}

##### Example

    cargo run --release video-ddm 80 1 simulations/Brownian_example.avi Brownian_example_results.csv

#### Python automation script
Saves to folder results and results-transposed and uploads data to crowd.sqlite database.

Activate Virtual Environment

    source .venv/bin/activate

Python run script commands

    python3 run.py video-ddm {frame_buffer_capacity} {annuli_spacing} {videos_directory_path}
    python3 run.py video-multi-ddm {frame_buffer_capacity} {annuli_spacing} {videos_directory_path}
    python3 run.py add-to-db {database_file} {folder_containing_results} {filename_"radial_Avg.csv"} {database_table_prefix}
    python3 run.py fit {csvs folder path} 
    python3 run.py fit {csv file path} custom #fit to a custom function not just brownian and ballistic.
    python3 run.py plot #Search database for video and create a plot at specific values

