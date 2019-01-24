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