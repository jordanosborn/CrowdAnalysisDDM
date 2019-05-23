#!/bin/bash
cd $HOME
#Pre-requisites
sudo apt update && sudo apt upgrade -y
sudo apt install -y apt-utils sudo nano
sudo apt install -y make cmake git curl gcc g++ wget python3-pip libssl-dev pkg-config zlib1g-dev clang libc++-dev
sudo apt install -y libomp-dev libgmp-dev libboost-all-dev build-essential
sudo pip3 install --upgrade pip

#OpenCV4
sudo apt install -y libopencv-dev

#Arrayfire
sudo apt install -y libfreeimage-dev cmake-curses-gui
sudo apt install -y  libopenblas-dev libfftw3-dev liblapacke-dev libblas-dev libclblas-dev opencl-headers libboost-all-dev ocl-icd-opencl-dev
sudo apt install -y libglfw3-dev libfontconfig1-dev libglm-dev
git clone --recursive https://github.com/arrayfire/arrayfire.git
cd $HOME/arrayfire
git checkout 3.6
git submodule init && git submodule update
mkdir $HOME/arrayfire/build
cd $HOME/arrayfire/build
cmake .. -DCMAKE_BUILD_TYPE=Release -DCMAKE_HOST_COMPILER=clang
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

#Python Deps
cd $HOME
curl -sSL https://raw.githubusercontent.com/sdispater/poetry/master/get-poetry.py | python3
echo -e "source $HOME/.profile" >> $HOME/.bashrc
source $HOME/.bashrc

#Clone Repo
git clone https://github.com/jordanosborn/CrowdAnalysisDDM.git
cd $HOME/CrowdAnalysisDDM/code
git pull
poetry install
cargo test
