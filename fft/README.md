cargo clean if having linking isssues
install arrayfire.sh
bashrc
export AF_PATH=af root dir
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$AF_PATH/lib

Must compile OpenCV
Must compile ArrayFire
See docs


Write a cpp helper file that streams video using opencv and can send frames in to rust to be processed

https://doc.rust-lang.org/nomicon/ffi.html
https://doc.rust-lang.org/cargo/reference/build-scripts.html
wget https://github.com/opencv/opencv/archive/4.0.1.zip
https://docs.opencv.org/4.0.1/d7/d9f/tutorial_linux_install.html
https://github.com/arrayfire/arrayfire/wiki/Build-Instructions-for-OSX
Create set up script
Auto builds opencv4
Array fire 3.6
Install other dependencies etc.


install nano set AF_PATH
startup script docker
setup ssh  then git pull on start cd in to directory remove copying move docker stuff into separate folder

## On mac
brew install g++ gcc
(version 8)

compile opencv

with
export CC=/usr/local/bin/gcc-8
export CXX=/usr/local/bin/g++-8


https://docs.opencv.org/4.0.1/d7/d9f/tutorial_linux_install.html

install arrayfire using osx package
https://arrayfire.s3.amazonaws.com/3.6.2/ArrayFire-v3.6.2_OSX_x86_64.pkg