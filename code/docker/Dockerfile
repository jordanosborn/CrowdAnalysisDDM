FROM ubuntu:latest
ENV LANG C.UTF-8
ENV TZ=Europe/London
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
ENV DEBIAN_FRONTEND=noninteractive

WORKDIR /home
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y apt-utils sudo nano
RUN apt-get install -y make cmake git curl gcc g++ wget python3-pip libssl-dev pkg-config zlib1g-dev clang libc++-dev

#Opencv4
RUN apt-get install -y libopencv-dev

RUN apt-get install -y libomp-dev libgmp-dev libboost-all-dev
RUN pip3 install --upgrade pip
RUN apt-get install -y build-essential libgtk2.0-dev pkg-config libavcodec-dev libavformat-dev libswscale-dev
RUN apt-get install -y python-dev python-numpy libtbb2 libtbb-dev libjpeg-dev libpng-dev libtiff-dev libdc1394-22-dev
# RUN git clone https://github.com/opencv/opencv.git
# WORKDIR /home/opencv/build
# RUN cmake -D CMAKE_BUILD_TYPE=Release -D CMAKE_INSTALL_PREFIX=/usr/local ..
# RUN make -j4
# RUN make install

# #Arrayfire no compile?
# RUN wget https://arrayfire.s3.amazonaws.com/3.6.2/ArrayFire-v3.6.2_Linux_x86_64.sh
# RUN apt-get install -y build-essential libfreeimage3 libfontconfig1 libglu1-mesa
# RUN chmod +x ./ArrayFire-v3.6.2_Linux_x86_64.sh
# RUN ./ArrayFire-v3.6.2_Linux_x86_64.sh --include-subdir --prefix=/opt
# RUN echo /opt/arrayfire/lib64 > /etc/ld.so.conf.d/arrayfire.conf
# RUN ldconfig

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
RUN make -j4
RUN make install

#Other programs
RUN ldconfig

RUN useradd -rm -d /home/ubuntu -s /bin/bash -g root -G sudo -u 1000 -p ubuntu ubuntu
RUN echo ubuntu:ubuntu | chpasswd
USER ubuntu
WORKDIR /home/ubuntu

RUN git clone https://github.com/jordanosborn/MastersProject.git
WORKDIR /home/ubuntu/MastersProject
#rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
# RUN /home/ubuntu/.cargo/bin/rustup install nightly
# RUN /home/ubuntu/.cargo/bin/rustup default stable
RUN /home/ubuntu/.cargo/bin/rustup component add rls rustfmt clippy

RUN RUSTFLAGS="--cfg procmacro2_semver_exempt" /home/ubuntu/.cargo/bin/cargo install cargo-tarpaulin
COPY ./docker_startup.sh .


ENTRYPOINT ["/bin/bash", "./docker_startup.sh"]
