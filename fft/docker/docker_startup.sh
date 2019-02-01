#!/bin/bash

cd $HOME
source $HOME/.cargo/env
rustup self update
rustup update
#echo -e "export AF_PATH='/opt/arrayfire'" >> $HOME/.bashrc
echo -e "export AF_PATH='/usr/local'" >> $HOME/.bashrc
#echo -e "export LD_LIBRARY_PATH='/opt/arrayfire/lib64'" >> $HOME/.bashrc
source $HOME/.bashrc
cd MastersProject
git pull

cd fft

cargo test
if [[ $? -ne 0 ]]
then
  exit $?
fi

cargo clippy --all-targets --all-features -- -D warnings
if [[ $? -ne 0 ]]
then
  exit $?
fi
