#include "../include/vidstream.hpp"

extern "C" int add(int x, int y) {
    return x + y;
}

extern "C" void start_capture(char* filename, int size) {
    std::cout << filename[size - 1] << std::endl;
}