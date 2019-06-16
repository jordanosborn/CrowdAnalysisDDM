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
    
#### Docker (run natively if you can)
[Dockerfile](https://github.com/jordanosborn/CrowdAnalysisDDM/raw/master/code/docker/Dockerfile)

Run using a startup script or attach a bash session and run using commands below

Example **docker_startup.sh** assuming repo is located at $HOME/CrowdAnalysisDDM

    #!/bin/bash
    cd $HOME/CrowdAnalysisDDM/code
    # git pull && rustup self update && rustup update
    cargo run --release "$@"

Note: Dockerfile and docker_startup.sh file should be in the same folder. Script will pass the arguments given to the **docker run** command to the DDM executable. 

All results will be stored inside the container can be extracted using

    docker cp <containerId>:/file/path/within/container /host/path/target

New containers will be created each time you use docker run (take up space), unless you restart and attach to an existing container.

To run ensure you are in the directory containing both the Dockerfile and docker_startup.sh

    docker build -t ddm:latest .
    docker run -ti --name {container_name} ddm:latest {arguments}

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

