#ifndef VIDSTREAM_H
#define VIDSTREAM_H
#include <thread>
#include <iostream>

#include "opencv2/opencv.hpp"
using Pixel = cv::Point3_<uint8_t>;

extern "C" int get_frame(int, int);

extern "C" void start_capture(char*, int);

#endif