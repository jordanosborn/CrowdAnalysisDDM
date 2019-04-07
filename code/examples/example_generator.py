import cv2
import sys
import numpy as np
from typing import Tuple, List

WHITE = (255, 255, 255)


def set_particle(img: np.array, L: int, location: Tuple[int, int]) -> np.array:
    px, py = location
    px_p1, py_p1 = (px + 1) % L, (py + 1) % L
    px_m1, py_m1 = (px - 1) % L, (py - 1) % L
    px, py = px % L, py % L
    img[px][py] = WHITE
    img[px_p1][py] = WHITE
    img[px][py_p1] = WHITE
    img[px_p1][py_p1] = WHITE
    img[px_m1][py] = WHITE
    img[px][py_m1] = WHITE
    img[px_m1][py_m1] = WHITE
    img[px_p1][py_m1] = WHITE
    img[px_m1][py_p1] = WHITE
    return img


def brownian(N, t, mu: Tuple[float, float], sigma: Tuple[float, float], dt=0.1):
    return zip(
        np.sqrt(dt) * sigma[0] * np.random.normal(mu[0], sigma[0], N),
        np.sqrt(dt) * sigma[1] * np.random.normal(mu[1], sigma[1], N),
    )


def brownian_drift(N, t, mu: Tuple[float, float], sigma: Tuple[float, float], dt=0.1):
    return zip(
        np.sqrt(dt) * sigma[0] * np.random.normal(mu[0] * t, sigma[0] * t ** 2, N),
        np.sqrt(dt) * sigma[1] * np.random.normal(mu[1] * t, sigma[1] * t ** 2, N),
    )


class particle:
    def __init__(self, L, dL=10):
        self.x = np.random.uniform(dL, L - dL)
        self.y = np.random.uniform(dL, L - dL)

    def update(self, L, dx, dy):
        self.x = self.x + dx
        self.y = self.y + dy


if __name__ == "__main__":
    t, dt = 0, 0.1
    fps, frames = 2 / dt, 200
    N, L, dL = 1000, 800, 10
    mu_x, mu_y = 0, 1
    sigma_x, sigma_y = 1, 1
    video = cv2.VideoWriter(sys.argv[1], cv2.VideoWriter_fourcc(*"H264"), fps, (L, L))
    particles = list(map(lambda _: particle(L, dL), range(N)))
    for _ in range(frames):
        img = np.zeros((L, L, 3), np.uint8)
        delta = list(brownian(N, t, (mu_x, mu_y), (sigma_x, sigma_y), dt))
        for p, d in zip(particles, delta):
            px, py = int(p.x), int(p.y)
            img = set_particle(img, L, (px, py))
            p.update(L, *d)
        t += dt
        video.write(img)
    cv2.destroyAllWindows()
    video.release()
