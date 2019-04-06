import cv2
import numpy as np

WHITE = (255, 255, 255)


def brownian(N, t, mu, sigma, dt=0.1):
    return zip(
        np.sqrt(dt)
        * sigma
        * np.random.normal(mu * t, sigma * (t ** 2 if mu != 0 else 1), N),
        np.sqrt(dt)
        * sigma
        * np.random.normal(mu * t, sigma * (t ** 2 if mu != 0 else 1), N),
    )


class particle:
    def __init__(self, L, dL=10):
        self.x = np.random.uniform(dL, L - dL)
        self.y = np.random.uniform(dL, L - dL)

    def update(self, L, dx, dy):
        self.x = (self.x + dx) % L
        self.y = (self.y + dy) % L


if __name__ == "__main__":
    t, dt = 0, 0.1
    fps, frames = 2 / dt, 200
    N, L, dL = 1000, 800, 10
    mu, sigma = 0, 3
    video = cv2.VideoWriter("out.avi", cv2.VideoWriter_fourcc(*"H264"), fps, (L, L))
    particles = map(lambda _: particle(L, dL), range(N))
    for _ in range(frames):
        img = np.zeros((L, L, 3), np.uint8)
        delta = brownian(N, t, mu, sigma, dt)
        for p, d in zip(particles, delta):
            px, py = int(p.x) % L, int(p.y) % L
            px_p1, py_p1 = (px + 1) % L, (py + 1) % L
            px_m1, py_m1 = (px - 1) % L, (py - 1) % L
            img[px][py] = WHITE
            img[px_p1][py] = WHITE
            img[px][py_p1] = WHITE
            img[px_p1][py_p1] = WHITE
            img[px_m1][py] = WHITE
            img[px][py_m1] = WHITE
            img[px_m1][py_m1] = WHITE
            img[px_p1][py_m1] = WHITE
            img[px_m1][py_p1] = WHITE
            p.update(L, *d)
        t += dt
        video.write(img)
    cv2.destroyAllWindows()
    video.release()
