FROM ubuntu:latest
ENV LANG C.UTF-8

WORKDIR /home

#Opencv4
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y apt-utils sudo
RUN apt-get install -y python3-pip libomp-dev libgmp-dev libboost-all-dev cmake make git wget curl gcc g++
RUN pip3 install --upgrade pip
RUN apt-get install -y build-essential libgtk2.0-dev pkg-config libavcodec-dev libavformat-dev libswscale-dev
RUN apt-get install -y python-dev python-numpy libtbb2 libtbb-dev libjpeg-dev libpng-dev libtiff-dev libdc1394-22-dev
RUN git clone https://github.com/opencv/opencv.git
WORKDIR /home/opencv/build
RUN cmake -D CMAKE_BUILD_TYPE=Release -D CMAKE_INSTALL_PREFIX=/usr/local ..
RUN make -j8
RUN make install

#Arrayfire
RUN apt-get install -y libfreeimage-dev cmake-curses-gui
RUN apt-get install -y libopenblas-dev libfftw3-dev liblapacke-dev opencl-headers libboost-all-dev ocl-icd-opencl-dev
RUN apt-get install -y libglfw3-dev libfontconfig1-dev libglm-dev
WORKDIR /home
RUN git clone --recursive https://github.com/arrayfire/arrayfire.git
WORKDIR /home/arrayfire
RUN git checkout v3.6
RUN git submodule init && git submodule update
WORKDIR /home/arrayfire/build
RUN cmake .. -DCMAKE_BUILD_TYPE=Release
RUN make -j8 
RUN make install

#Other programs
RUN apt-get install -y nano
RUN ldconfig

RUN useradd -rm -d /home/ubuntu -s /bin/bash -g root -G sudo -u 1000 -p ubuntu ubuntu
RUN echo ubuntu:ubuntu | chpasswd
USER ubuntu
WORKDIR /home/ubuntu

#rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN /home/ubuntu/.cargo/bin/rustup install nightly
RUN /home/ubuntu/.cargo/bin/rustup default stable
RUN /home/ubuntu/.cargo/bin/rustup component add rls rustfmt clippy

COPY . "/home/ubuntu/"
ENTRYPOINT ["bash", "./docker_startup.sh"]
#CMD ["source", ".cargo/env"]
#CMD ["rustup", "self", "update"]
#CMD ["rustup", "update"]
#CMD ["bash"]
