#!/usr/bin/env python3
from data_clean import data_open
from sys import argv
from matplotlib import pyplot as plt
import numpy as np
from scipy.optimize import curve_fit


def func(x, a, b, c):
    return a * np.exp(-x / b) + c


def get_fit(f, x, y):
    fit, _ = curve_fit(func, x, y, bounds=([-np.inf, 0.0, -np.inf], np.inf))
    return fit


# TODO: save csv with tau_c vs q for chosen video
if __name__ == "__main__":
    index, x_data, Y = data_open(argv[1])
    x_data = np.array(x_data, dtype=np.float)
    y_data = np.array(Y[int(argv[2])], dtype=np.float)
    y_data = y_data / np.max(y_data)
    fit = get_fit(func, x_data, y_data)
    tau_c = fit[1]
    print(tau_c)
    plt.plot(x_data, func(x_data, *fit))
    plt.plot(x_data, y_data)
    plt.show()
