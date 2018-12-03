#include <iostream>
#include "opencv2/opencv.hpp"


int main() {
    std::cout << "Hello, World!" << std::endl;
    cv::VideoCapture stream1(0);   //0 is the id of video device.0 if you have only one camera.

    if (!stream1.isOpened()) { //check if video device has been initialised
        std::cout << "cannot open camera";
    }

    while (true) {
        cv::Mat cameraFrame;
        stream1.read(cameraFrame);
        cv::imshow("cam", cameraFrame);
        if (cv::waitKey(30) >= 0)
            break;
    }

    return 0;
}