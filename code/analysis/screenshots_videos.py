import cv2
import sys
import os

if __name__ == "__main__":
    directory = sys.argv[1]
    if not os.path.exists("screenshots"):
        os.mkdir("screenshots")
    files = [os.path.join(directory, s) for s in os.listdir(directory)]
    for f in files:
        name = os.path.basename(os.path.splitext(f)[0])
        stream = cv2.VideoCapture(f)
        success, image = stream.read()
        count = 0
        while success:
            success, image = stream.read()
            count += 1
            if count == 10:
                cv2.imwrite(f"screenshots/{name}.png", image)
                print(f"Saved {name}.png")
