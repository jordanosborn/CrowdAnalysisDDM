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

extern "C" void *get_frame(size_t);

extern "C" size_t start_capture(const char*);

extern "C" size_t start_camera_capture();

extern "C" void show(const cv::Mat&);

extern "C" void show_next(size_t stream_id);

extern "C" void *mat_new();

extern "C" int mat_rows(const cv::Mat*);

extern "C" int mat_cols(const cv::Mat*);

extern "C" int mat_depth(const cv::Mat*);

extern "C" int mat_channels(const cv::Mat*);

extern "C" const uint8_t *mat_data(const cv::Mat*);

extern "C" void mat_drop(cv::Mat*);


extern "C" int mat_type(const cv::Mat* const);

extern "C" const uint8_t* mat_data(const cv::Mat* const);

extern "C" size_t mat_total(const cv::Mat* const);

extern "C" size_t mat_elem_size(const cv::Mat* const);

extern "C" size_t mat_elem_size1(const cv::Mat* const);

extern "C" size_t mat_step1(const cv::Mat* const, int);


#endif