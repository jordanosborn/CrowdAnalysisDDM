cmake_minimum_required(VERSION 3.12)
project(Code)

include_directories(./include)
find_package(OpenCV REQUIRED)

if (${APPLE})
    link_directories(/usr/local/lib/)
    set(CMAKE_C_COMPILER "/usr/local/Cellar/llvm/7.0.0_1/bin/clang")
    set(CMAKE_CXX_COMPILER "//usr/local/Cellar/llvm/7.0.0_1/bin/clang++")
elseif ($(UNIX))

elseif(${WIN32})

endif()


include_directories(${OpenCV_INCLUDE_DIRS})
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -pedantic -Wextra -Ofast -flto -fopenmp -lgmp -D_GLIBCXX_PARALLEL -march=native")
include_directories(${GOBJECT_INCLUDE_DIR})

file(GLOB_RECURSE source_files "src/*.cpp")


set(CMAKE_CXX_STANDARD 17)
add_executable(Code main.cpp ${source_files})
target_link_libraries(Code ${OpenCV_LIBS})