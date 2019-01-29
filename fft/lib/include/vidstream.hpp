#ifndef VIDSTREAM_H
#define VIDSTREAM_H
#include <thread>
#include <iostream>
#include <vector>
#include <stdlib.h>
#include "opencv2/core.hpp"
#include "opencv2/opencv.hpp"
#include <opencv2/highgui.hpp>


static std::vector<cv::VideoCapture> streams;

using Pixel = cv::Point3_<uint8_t>;

extern "C" cv::Mat* get_frame(size_t);

extern "C" size_t start_capture(const char*);

extern "C" size_t start_camera_capture();

extern "C" void show(const cv::Mat&);

extern "C" void show_next(size_t stream_id);

#endif