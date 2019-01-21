cmake_minimum_required(VERSION 3.12)
project(Code LANGUAGES CXX)#CUDA)


include_directories(./include)
find_package(OpenCV REQUIRED)

# Glog opencv openmp llvm gmp

if (${APPLE})
    link_directories(/usr/local/lib/)
    include_directories(/usr/local/include)
    set(CMAKE_C_COMPILER "/usr/local/Cellar/llvm/7.0.1/bin/clang")
    set(CMAKE_CXX_COMPILER "/usr/local/Cellar/llvm/7.0.1/bin/clang++")
elseif ($(UNIX))
    set(CMAKE_C_COMPILER "gcc")
    set(CMAKE_CXX_COMPILER "g++")
    set(CMAKE_CUDA_HOST_COMPILER "gcc")
elseif(${WIN32})

endif()


include_directories(${OpenCV_INCLUDE_DIRS})
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -pedantic -Wextra -Ofast -flto -fopenmp -lgmp -lgsl -D_GLIBCXX_PARALLEL -march=native")
include_directories(${GOBJECT_INCLUDE_DIR})

file(GLOB_RECURSE source_files "src/*.cpp")


set(CMAKE_CXX_STANDARD 17)
add_executable(Code main.cpp ${source_files})
target_link_libraries(Code ${OpenCV_LIBS})