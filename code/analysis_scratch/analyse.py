#!/usr/bin/env python3
from data_clean import data_open
from sys import argv
from matplotlib import pyplot as plt
import numpy as np
from scipy.optimize import curve_fit
from multiprocessing.pool import Pool
import os


def func(x, a, b, c):
    return a * np.exp(-x / b) + c


def get_fit(f, x, y):
    fit, _ = curve_fit(func, x, y, bounds=([-np.inf, 0.0, -np.inf], np.inf))
    return fit


def create_plot(path: str):
    print(path)
    index, x_data, Y = data_open(path + "/radial_Avg.csv")
    x_data = np.array(x_data)
    data = []

    # Save all plots of I vs tau for each q
    for q, y in zip(index, Y):
        y_data = np.array(y)
        y_max = np.max(y_data)
        fit = get_fit(func, x_data, y_data / y_max)
        fit = [fit[0] * y_max, fit[1], fit[2] * y_max]
        data.append(fit)
        plt.title(
            f"Plot of Intensity difference (normalised by I_max) for q={q} against frame difference tau"
        )
        plt.ylabel(f"I(q={q}, tau)")
        plt.xlabel("tau")
        plt.plot(x_data, y_data, label="data")
        plt.plot(
            x_data,
            func(x_data, *fit),
            label=f"fit {round(fit[0], 2)}*exp(-tau/{round(fit[1], 2)}) + {round(fit[2], 2)}",
        )
        plt.legend(loc="upper left")
        plt.savefig(f"{path}/I_vs_tau_for_q_{q}.png")

    # # Save raw fit data
    

    # plt.title(
    #         f"Plot of Intensity difference (normalised by I_max) for q={q} against frame difference tau"
    #     )
    # plt.ylabel(f"I(q={q}, tau)")
    # plt.xlabel("tau")
    # plt.plot(x_data, y_data, label="data")
    # plt.plot(
    #     x_data,
    #     func(x_data, *fit),
    #     label=f"fit {round(fit[0], 2)}*exp(-tau/{round(fit[1], 2)}) + {round(fit[2], 2)}",
    # )
    # plt.legend(loc="upper left")
    # plt.savefig(f"{path}/I_vs_tau_for_q_{q}.png")


# TODO: save csv with tau_c vs q for chosen video
if __name__ == "__main__":
    if os.path.isdir(argv[1]):
        files = []
        for (dirpath, dirnames, filenames) in os.walk(argv[1]):
            files.extend(map(lambda s: f"./{dirpath}/{s}", filenames))
        files = list(filter(lambda s: s.find("radial_Avg.csv") != -1, files))
        output = list(map(lambda s: s.replace("/radial_Avg.csv", ""), files))
        print(output)
        for path in output:
            create_plot(path)

    # index, x_data, Y = data_open(argv[1])
    # x_data = np.array(x_data, dtype=np.float)
    # y_data = np.array(Y[int(argv[2])], dtype=np.float)
    # y_data = y_data / np.max(y_data)
    # fit = get_fit(func, x_data, y_data)
    # tau_c = fit[1]
    # print(tau_c)
    # plt.plot(x_data, func(x_data, *fit))
    # plt.plot(x_data, y_data)
    # plt.show()
