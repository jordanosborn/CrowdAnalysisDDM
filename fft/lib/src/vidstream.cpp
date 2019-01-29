#include "../include/vidstream.hpp"

size_t start_capture(const char* filename) {
    auto stream = cv::VideoCapture(filename);
    if (!stream.isOpened()) {
        std::cout << "Failed to open stream - " << filename << "\n";
        exit(-1);
    }
    streams.push_back(stream);
    return streams.size() - 1;
}

size_t start_camera_capture() {
    auto stream = cv::VideoCapture(0);
    if (!stream.isOpened()) {
        std::cout << "Failed to open stream - " << 0 << "\n";
        exit(-1);
    }
    streams.push_back(stream);
    return streams.size() - 1;
}

void *get_frame(size_t stream_id) {
    cv::Mat *frame;
    if (!streams[stream_id].read(*frame)) {
        return NULL;
    }
    cv::flip(*frame, *frame, 1);
    cv::cvtColor(*frame, *frame, cv::COLOR_BGR2RGB);
    return (void*)frame;
}

void show(const cv::Mat& frame) {
    cv::imshow("cam", frame);
}

void *mat_new() {
    cv::Mat *ptr = new cv::Mat();
    return (ptr);
}

int mat_rows(const cv::Mat* matrix) {
    return matrix->rows;
}

int mat_cols(const cv::Mat* matrix) {
    return matrix->cols;
}

int mat_depth(const cv::Mat* matrix) {
    return matrix->depth();
}

int mat_channels(const cv::Mat* matrix) {
    return matrix->channels();
}

void mat_drop(cv::Mat *matrix) {
    delete matrix;
    matrix = nullptr;
}
