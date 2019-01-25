#!/bin/bash

cd $HOME
source $HOME/.cargo/env
rustup self update
rustup update
echo -e "export AF_PATH='/opt/arrayfire'" >> $HOME/.bashrc
echo -e "export LD_LIBRARY_PATH='$LD_LIBRARY_PATH:$AF_PATH/lib64'" >> $HOME/.bashrc
source $HOME/.bashrc


bash
