#!/bin/bash

exit_if_failed() {
  if [[ $1 -ne 0 ]]; then
    exit $1
  fi
}

source $HOME/.bashrc
source $HOME/.cargo/env

cd $HOME/CrowdAnalysisDDM/code

rustup self update
rustup update
ldconfig
git pull

cargo clippy --all-targets --all-features -- -D warnings
exit_if_failed $?

exit 0
