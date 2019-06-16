FROM ubuntu:18.04
ENV LANG C.UTF-8
ENV TZ=Europe/London
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
ENV DEBIAN_FRONTEND=noninteractive

WORKDIR /home

#Pre-requisites (build tools etc.)
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y apt-utils sudo nano virtualenv
RUN apt-get install -y make cmake git curl gcc g++ wget python3-pip libssl-dev openssl pkg-config zlib1g-dev clang libc++-dev
RUN apt-get install -y libomp-dev libgmp-dev libboost-all-dev build-essential
RUN pip3 install --upgrade pip

#Install OpenCV and pre-requisites
RUN apt-get install -y ffmpeg libavcodec-dev libavformat-dev libavdevice-dev
RUN apt-get install -y libopencv-dev
RUN pip3 install virtualenv opencv-python

#Use Latest cmake arrayfire build fails on cmake version < 3.11
WORKDIR /home/DDM
RUN git clone https://github.com/Kitware/CMake.git
WORKDIR /home/DDM/CMake/build
RUN cmake .. -DCMAKE_BUILD_TYPE:STRING=Release -DCMAKE_INSTALL_PREFIX=$HOME/DDM
RUN make -j4
RUN make install

#ArrayFire dependencies and clone repository
WORKDIR /home/DDM
RUN apt-get install -y libfreeimage-dev cmake-curses-gui
RUN apt-get install -y libopenblas-dev libatlas-base-dev libfftw3-dev liblapacke-dev libblas-dev libclblas-dev opencl-headers libboost-all-dev ocl-icd-opencl-dev
RUN apt-get install -y libglfw3-dev libfontconfig1-dev libglm-dev
RUN git clone --recursive https://github.com/arrayfire/arrayfire.git
WORKDIR /home/DDM/arrayfire
RUN git checkout v3.6
RUN git submodule init && git submodule update

#Build Arrayfire
WORKDIR /home/DDM/arrayfire/build
RUN $HOME/DDM/bin/cmake .. -DCMAKE_BUILD_TYPE=Release -DCMAKE_HOST_COMPILER=clang
#-DAF_BUILD_CPU=OFF
RUN make -j4
RUN make install
RUN ldconfig
#Remove build folder
RUN rm -rf /home/DDM

#Add user ubuntu
RUN useradd -rm -d /home/ubuntu -s /bin/bash -g root -G sudo -u 1000 -p ubuntu ubuntu
RUN echo ubuntu:ubuntu | chpasswd
USER ubuntu

#Set arrayfire Env variables
WORKDIR /home/ubuntu
RUN echo -e "export AF_PATH='/usr/local'" >> .bashrc
RUN echo -e "export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH" >> .bashrc

#Install Rust and cargo components
WORKDIR /home/ubuntu
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN .cargo/bin/rustup component add rls rustfmt clippy
RUN echo -e "source $HOME/.cargo/env" >> .bashrc

#Install Poetry
RUN curl -sSL https://raw.githubusercontent.com/sdispater/poetry/master/get-poetry.py | python3

#Clone and configure repo (install python dependencies, build program etc.)
WORKDIR /home/ubuntu
RUN git clone https://github.com/jordanosborn/CrowdAnalysisDDM.git
WORKDIR /home/ubuntu/CrowdAnalysisDDM/code
RUN virtualenv -p python3 .venv
RUN /home/ubuntu/.poetry/bin/poetry update
RUN /home/ubuntu/.poetry/bin/poetry install

#Set entrypoint
WORKDIR /home/ubuntu
COPY ./docker_startup.sh .
ENTRYPOINT ["/bin/bash", "./docker_startup.sh"]
