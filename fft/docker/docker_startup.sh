#!/bin/bash

exit_if_failed() {
  if [[ $1 -ne 0 ]] then
    exit $1
  fi
}

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
exit_if_failed $?

cargo clippy --all-targets --all-features -- -D warnings
exit_if_failed $?
exit 0
