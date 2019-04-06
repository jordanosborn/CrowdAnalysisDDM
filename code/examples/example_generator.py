import cv2
import numpy as np

t = 0
dt = 0.1
fps = 2 / dt
N = 1000
L = 800
mu = 0
sigma = 3
frames = 200

WHITE = (255, 255, 255)
dL = 10


def brownian():
    return zip(
        np.sqrt(dt) * sigma * np.random.normal(mu * t, sigma, N),
        np.sqrt(dt) * sigma * np.random.normal(mu * t, sigma, N),
    )


class particle:
    def __init__(self):
        self.x = np.random.uniform(dL, L - dL)
        self.y = np.random.uniform(dL, L - dL)

    def update(self, dx, dy):
        self.x = (self.x + dx) % L
        self.y = (self.y + dy) % L


if __name__ == "__main__":
    video = cv2.VideoWriter("out.avi", cv2.VideoWriter_fourcc(*"H264"), fps, (L, L))
    particles = map(lambda _: particle(), range(N))
    for _ in range(frames):
        img = np.zeros((L, L, 3), np.uint8)
        delta = brownian()
        for p, d in zip(particles, delta):
            px, py = int(p.x), int(p.y)
            img[px][py] = WHITE
            img[px + 1][py] = WHITE
            img[px][py + 1] = WHITE
            img[px + 1][py + 1] = WHITE
            img[px - 1][py] = WHITE
            img[px][py - 1] = WHITE
            img[px - 1][py - 1] = WHITE
            img[px + 1][py - 1] = WHITE
            img[px - 1][py + 1] = WHITE
            p.update(*d)
        t += dt
        video.write(img)
    cv2.destroyAllWindows()
    video.release()
