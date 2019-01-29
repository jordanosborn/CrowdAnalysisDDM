#include "../include/vidstream.hpp"

extern "C" size_t start_capture(const char* filename) {
    auto stream = cv::VideoCapture(filename);
    if (!stream.isOpened()) {
        std::cout << "Failed to open stream - " << filename << "\n";
        exit(-1);
    }
    streams.push_back(&stream);
    return streams.size() - 1;
}

extern "C" size_t start_camera_capture() {
    auto stream = cv::VideoCapture(0);
    if (!stream.isOpened()) {
        std::cout << "Failed to open stream - " << 0 << "\n";
        exit(-1);
    }
    streams.push_back(&stream);
    return streams.size() - 1;
}

extern "C" cv::Mat* get_frame(size_t stream_id) {
    auto stream = streams.at(stream_id);
    cv::Mat frame;
    if (!stream->read(frame)) {
        return NULL;
    }
    cv::flip(frame, frame, 1);
    cv::cvtColor(frame, frame, cv::COLOR_BGR2RGB);
    return &frame;
}

extern "C" void show(const cv::Mat& frame) {
    cv::imshow("cam", frame);
}


extern "C" void show_next(size_t stream_id) {
    cv::Mat frame;
    auto stream = streams.at(stream_id);
    stream->read(frame);
    //cv::imshow("cam", frame);
}