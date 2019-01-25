FROM ubuntu:latest
ENV LANG C.UTF-8

WORKDIR /home

#Opencv4
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y apt-utils
RUN apt-get install -y python3-pip libomp-dev libgmp-dev libboost-all-dev cmake make git wget curl gcc g++
RUN pip3 install --upgrade pip
RUN apt-get install -y build-essential libgtk2.0-dev pkg-config libavcodec-dev libavformat-dev libswscale-dev
RUN apt-get install -y python-dev python-numpy libtbb2 libtbb-dev libjpeg-dev libpng-dev libtiff-dev libjasper-dev libdc1394-22-dev
RUN git clone https://github.com/opencv/opencv.git
RUN cd opencv
RUN mkdir build && cd build
RUN cmake -D CMAKE_BUILD_TYPE=Release -D CMAKE_INSTALL_PREFIX=/usr/local ..
RUN make -j8
RUN make install

#Arrayfire
RUN apt-get install -y libfreeimage-dev cmake-curses-gui
RUN apt-get install -y libopenblas-dev libfftw3-dev liblapacke-dev opencl-headers libboost-all-dev ocl-icd-opencl-dev
RUN apt-get install -y libglfw3-dev libfontconfig1-dev libglm-dev
RUN cd ..
RUN git clone --recursive https://github.com/arrayfire/arrayfire.git
RUN cd arrayfire
RUN git checkout v3.6
RUN git submodule init && git submodule update
RUN mkdir build && cd build
RUN cmake .. -DCMAKE_BUILD_TYPE=Release
RUN make -j8 
RUN make install

RUN useradd -rm -d /home/ubuntu -s /bin/bash -g root -G sudo -u 1000 -p ubuntu ubuntu
USER ubuntu
WORKDIR /home/ubuntu
COPY "." "/hom/ubuntu/code/"

#rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN source ~/.cargo/env
RUN rustup install nightly
RUN rustup defauly nightly
RUN rustup component add rls rustfmt clippy

ENTRYPOINT [ "bash" ]
CMD ["sudo apt update && sudo apt upgrade"]

