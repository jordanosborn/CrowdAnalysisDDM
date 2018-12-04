#include <iostream>
#include "opencv2/opencv.hpp"
#include <thread>
#include <atomic>


int main() {
    cv::VideoCapture stream1(0);   //0 is the id of video device.0 if you have only one camera.

    if (!stream1.isOpened()) { //check if video device has been initialised
        std::cout << "cannot open camera";
        throw std::exception();
    }

    #pragma omp parallel
    {
    #pragma omp for
        for (int i=0; i<10; i++)
            std::cout << i;
    }

    while (true) {
        cv::Mat cameraFrame;
        stream1.read(cameraFrame);
        cv::flip(cameraFrame, cameraFrame, 1);

        cv::imshow("cam", cameraFrame);
        if (cv::waitKey(30) >= 0)
            break;
    }

    return 0;
}