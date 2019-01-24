#ifndef VIDSTREAM_H
#define VIDSTREAM_H
#include "opencv2/opencv.hpp"
#include <thread>
#include <iostream>

using Pixel = cv::Point3_<uint8_t>;

extern "C" int get_frame(int, int);

extern "C" void start_capture(char* filename);

#endif