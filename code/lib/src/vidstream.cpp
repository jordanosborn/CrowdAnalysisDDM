#include "../include/vidstream.hpp"

size_t start_capture(const char* filename) {
    cv::String f(filename);
    auto stream = cv::VideoCapture(f);
    if (!stream.isOpened()) {
        std::cout << "Failed to open stream - " << filename << "\n";
        exit(-1);
    }
    streams.push_back(stream);
    return streams.size() - 1;
}

size_t start_camera_capture() {
    auto stream = cv::VideoCapture(0);
    stream.set(cv::CAP_PROP_FORMAT, CV_8UC3);
    if (!stream.isOpened()) {
        std::cout << "Failed to open stream - " << 0 << "\n";
        exit(-1);
    }
    streams.push_back(stream);
    return streams.size() - 1;
}

void close_stream(size_t stream_id) {
    if (streams.size() > stream_id && streams[stream_id].isOpened())
        streams[stream_id].release();
}

void *get_frame(size_t stream_id) {
    cv::Mat frame;
    if (!streams[stream_id].read(frame)) {
        return NULL;
    }
    // Flips image so that it is in correct orientation
    cv::flip(frame, frame, 1);
    cv::transpose(frame, frame);
    cv::cvtColor(frame, frame, cv::COLOR_BGR2RGB);
    auto frame_ptr = new cv::Mat(frame);
    return (void*)frame_ptr;
}

void show(const cv::Mat& frame) {
    cv::imshow("cam", frame);
}

void show_next(size_t stream_id) {
    cv::Mat frame;
    cv::namedWindow("cam", cv::WINDOW_AUTOSIZE);
    while (streams[stream_id].read(frame)) {
        cv::imshow("cam", frame);
        cv::waitKey(30);
    }
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


int mat_type(const cv::Mat* const mat) {
    return mat->type();
}

const uint8_t* mat_data(const cv::Mat* const mat) {
    return mat->data;
}

size_t mat_total(const cv::Mat* const mat) {
    return mat->total();
}

size_t mat_elem_size(const cv::Mat* const mat) {
    return mat->elemSize();
}

size_t mat_elem_size1(const cv::Mat* const mat) {
    return mat->elemSize1();
}

size_t mat_step1(const cv::Mat* const mat, int i) {
    return mat->step1(i);
}

size_t get_fps(size_t stream_id) {
    if (streams.size() > stream_id && streams[stream_id].isOpened()) {
        return streams[stream_id].get(cv::CAP_PROP_FPS);
    } else {
        return 0;
    }
}
