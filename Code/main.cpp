#include <iostream>
#include "opencv2/opencv.hpp"
#include <thread>


using Pixel = cv::Point3_<uint8_t>;

inline auto avg(std::vector<cv::Mat> &v) {
    bool defined = false;
    cv::Mat s;
    for (const auto &e: v) {
        if (not defined) {
            s = e;
            defined = true;
        } else {
            s += e;
        }

    }
    return s / v.size();
}

int main() {
    cv::VideoCapture stream1("video.mp4");   //0 is the id of video device.0 if you have only one camera.
    if (!stream1.isOpened()) { //check if video device has been initialised
        std::cout << "cannot open camera";
        throw std::exception();
    }

//    #pragma omp parallel
//    {
//    #pragma omp for
//        for (int i=0; i<10; i++)
//            std::cout << i;
//    }

    std::vector<cv::Mat> avgFrame;
    int i = 0;

    while (true) {
        cv::Mat cameraFrame;
        stream1.read(cameraFrame);
        cv::flip(cameraFrame, cameraFrame, 1);
        cv::cvtColor(cameraFrame, cameraFrame, cv::COLOR_BGR2GRAY);
//        cameraFrame.forEach<Pixel>([](Pixel &px, const int *pos) {
//            if (pos[0] < 600 && pos[1] < 200) {
//                px.x = 0;
//                px.y = 0;
//            }
//        });
        if (avgFrame.size() != 10) {
            avgFrame.push_back(cameraFrame);
            cv::imshow("cam", cameraFrame);
        } else {
            avgFrame[i] = cameraFrame;
            cv::imshow("cam", cameraFrame - avg(avgFrame));
        }

        i++;
        if (i == 10) {
            i = 0;
        }
        if (cv::waitKey(30) >= 0) break;
    }

    return 0;
}