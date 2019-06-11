#!/bin/bash
mkdir $HOME/DDM
cd $HOME/DDM

#Pre-requisites
sudo apt update && sudo apt upgrade -y
sudo apt install -y apt-utils sudo nano virtualenv
sudo apt install -y make cmake git curl gcc g++ wget python3-pip libssl-dev openssl pkg-config zlib1g-dev clang libc++-dev
sudo apt install -y libomp-dev libgmp-dev libboost-all-dev build-essential
sudo pip3 install --upgrade pip
sudo pip3 install virtualenv

#OpenCV installation
sudo apt install -y libopencv-dev

#Arrayfire dependency installation and building
sudo apt install -y libfreeimage-dev cmake-curses-gui
sudo apt install -y libopenblas-dev libatlas-base-dev libfftw3-dev liblapacke-dev libblas-dev libclblas-dev opencl-headers libboost-all-dev ocl-icd-opencl-dev
sudo apt install -y libglfw3-dev libfontconfig1-dev libglm-dev
git clone --recursive https://github.com/arrayfire/arrayfire.git
cd $HOME/DDM/arrayfire
git checkout v3.6
git submodule init && git submodule update
mkdir $HOME/DDM/arrayfire/build

#Use latest cmake
cd $HOME/DDM/
git clone https://github.com/Kitware/CMake.git
cd $HOME/DDM/CMake
mkdir $HOME/DDM/CMake/build
cd $HOME/DDM/CMake/build
cmake .. -DCMAKE_BUILD_TYPE:STRING=Release -DCMAKE_INSTALL_PREFIX=$HOME/DDM
make -j4
make install

#Build arrayfire
cd $HOME/DDM/arrayfire/build
$HOME/DDM/bin/cmake .. -DCMAKE_BUILD_TYPE=Release -DCMAKE_HOST_COMPILER=clang -DAF_BUILD_CPU=OFF
make -j4
sudo make install
echo -e "export AF_PATH='/usr/local'" >> $HOME/.bashrc
echo -e "export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH" >> $HOME/.bashrc

cd $HOME
#Rust installation
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
echo -e "source $HOME/.cargo/env" >> $HOME/.bashrc
$HOME/.cargo/bin/rustup component add rls rustfmt clippy

#Poetry installation - python dependency management
cd $HOME/DDM
curl -sSL https://raw.githubusercontent.com/sdispater/poetry/master/get-poetry.py | python3

#Configure linking directories
source $HOME/.bashrc
sudo ldconfig

#Clone Repo
cd $HOME/DDM
git clone https://github.com/jordanosborn/CrowdAnalysisDDM.git
cd $HOME/DDM/CrowdAnalysisDDM/code
git pull

#Create python virtual environment and install all dependencies
virtualenv -p python3 .venv
source .venv/bin/activate
$HOME/.poetry/bin/poetry update
$HOME/.poetry/bin/poetry install

#Test that executable runs
cargo run
