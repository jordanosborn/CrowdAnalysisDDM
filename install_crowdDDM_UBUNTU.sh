#!/bin/bash
cd $HOME
#Pre-requisites
sudo apt update && sudo apt upgrade -y
sudo apt install -y apt-utils sudo nano
sudo apt install -y make cmake git curl gcc g++ wget python3-pip libssl-dev pkg-config zlib1g-dev clang libc++-dev

#OpenCV4
sudo apt install -y libopencv-dev
sudo apt install -y libomp-dev libgmp-dev libboost-all-dev
sudo pip3 install --upgrade pip
sudo apt install -y build-essential libgtk2.0-dev pkg-config libavcodec-dev libavformat-dev libswscale-dev
sudo apt install -y python-dev python-numpy libtbb2 libtbb-dev libjpeg-dev libpng-dev libtiff-dev libdc1394-22-dev

#Arrayfire
sudo apt install -y libfreeimage-dev cmake-curses-gui
sudo apt install -y libopenblas-dev libfftw3-dev liblapacke-dev opencl-headers libboost-all-dev ocl-icd-opencl-dev
sudo apt install -y libglfw3-dev libfontconfig1-dev libglm-dev
git clone --recursive https://github.com/arrayfire/arrayfire.git
cd $HOME/arrayfire
git checkout 3.6
git submodule init && git submodule update
cd $HOME/arrayfire/build
cmake .. -DCMAKE_BUILD_TYPE=Release
make -j4
sudo make install
echo -e "export AF_PATH='/usr/local'" >> $HOME/.bashrc
echo -e "export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH" >> $HOME/.bashrc
source $HOME/.bashrc
sudo ldconfig

cd $HOME
#Rust
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
$HOME/.cargo/bin/rustup component add rls rustfmt clippy

#Clone Repo
git clone https://github.com/jordanosborn/CrowdAnalysisDDM.git
cd $HOME/CrowdAnalysisDDM
cargo test
