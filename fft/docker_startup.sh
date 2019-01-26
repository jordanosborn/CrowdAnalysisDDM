#!/bin/bash

cd $HOME
source $HOME/.cargo/env
rustup self update
rustup update
echo -e "export AF_PATH='/usr/local'" >> $HOME/.bashrc
echo -e "export LD_LIBRARY_PATH='$LD_LIBRARY_PATH:$AF_PATH/lib'" >> $HOME/.bashrc
source $HOME/.bashrc
git pull

bash
