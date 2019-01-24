cargo clean if having linking isssues 
install arrayfire.sh
bashrc
export AF_PATH=af root dir
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$AF_PATH/lib

Write a cpp helper file that streams video using opencv and can send frames in to rust to be processed

https://doc.rust-lang.org/nomicon/ffi.html
https://doc.rust-lang.org/cargo/reference/build-scripts.html