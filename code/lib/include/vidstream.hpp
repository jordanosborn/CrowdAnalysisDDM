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

extern "C" {
    void *get_frame(size_t);

    size_t start_capture(const char*);

    size_t start_camera_capture();

    void show(const cv::Mat&);

    void write(const char*, const cv::Mat*);

    void show_next(size_t stream_id);

    void *mat_new();

    int mat_rows(const cv::Mat*);

    int mat_cols(const cv::Mat*);

    int mat_depth(const cv::Mat*);

    int mat_channels(const cv::Mat*);

    void mat_drop(cv::Mat*);

    int mat_type(const cv::Mat* const);

    const uint8_t* mat_data(const cv::Mat* const);

    size_t mat_total(const cv::Mat* const);

    size_t mat_elem_size(const cv::Mat* const);

    size_t mat_elem_size1(const cv::Mat* const);

    size_t mat_step1(const cv::Mat* const, int);

    size_t get_fps(size_t);

    void close_stream(size_t);

}
#endif