Cargo clean if having linking issues
install arrayfire.sh
bashrc
export AF_PATH=af root dir
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$AF_PATH/lib

Must compile OpenCV
Must compile ArrayFire
See docs


https://doc.rust-lang.org/nomicon/ffi.html
https://doc.rust-lang.org/cargo/reference/build-scripts.html
wget https://github.com/opencv/opencv/archive/4.0.1.zip
https://docs.opencv.org/4.0.1/d7/d9f/tutorial_linux_install.html
https://github.com/arrayfire/arrayfire/wiki/Build-Instructions-for-OSX
Create set up script
Auto builds opencv4
Array fire 3.6
Install other dependencies etc.


Install nano set AF_PATH
Startup script docker
setup ssh  then git pull on start cd in to directory remove copying move docker stuff into separate folder

## On mac
brew install opencv llvm (need clang-7) glfw

sudo update_dyld_shared_cache

use open /Library/Developer/CommandLineTools/Packages/macOS_SDK_headers_for_macOS_10.14.pkg
to fix stdlib errors on mac

install arrayfire using osx package
https://arrayfire.s3.amazonaws.com/3.6.2/ArrayFire-v3.6.2_OSX_x86_64.pkg

## On Linux
install arrayfire using linux package

move to /opt/arrayfire

install clang

install libstdc++

echo -e "export AF_PATH='/opt/arrayfire'" >> $HOME/.bashrc

echo -e "export LD_LIBRARY_PATH='/opt/arrayfire/lib64'" >> $HOME/.bashrc
