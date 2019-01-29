#!/bin/bash

cd $HOME
source $HOME/.cargo/env
rustup self update
rustup update
#echo -e "export AF_PATH='/opt/arrayfire'" >> $HOME/.bashrc
echo -e "export AF_PATH='/usr/local'" >> $HOME/.bashrc
#echo -e "export LD_LIBRARY_PATH='/opt/arrayfire/lib64'" >> $HOME/.bashrc
source $HOME/.bashrc
sudo ldconfig
cd MastersProject
git pull

cd fft
# cargo test
# cargo clippy --all-targets --all-features -- -D warnings
bash
