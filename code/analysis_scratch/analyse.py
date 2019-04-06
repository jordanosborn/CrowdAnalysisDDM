#!/usr/bin/env python3
from data_clean import data_open
from sys import argv
from matplotlib import pyplot as plt
import numpy as np
from scipy.optimize import curve_fit
import os
from twilio.rest import Client
from typing import Any, List
import json

with open("secrets.json") as f:
    secrets = json.loads(f.read())


def send_message(secrets: Any, body: str):
    account_sid = secrets["account_sid"]
    auth_token = secrets["auth_token"]
    client = Client(account_sid, auth_token)

    message = client.messages.create(
        body=body, from_=f'{secrets["twilio_number"]}', to=f'{secrets["phone_number"]}'
    )
    print(f'Sent message to {secrets["phone_number"]} message_ID = {message.sid}')


def func(x, a, b, c):
    return a * np.exp(-x / b) + c


def get_fit(f, x, y):
    try:
        fit, _ = curve_fit(func, x, y, bounds=([-np.inf, 0.0, -np.inf], np.inf))
    except RuntimeError:
        return [0.0, 1.0, 0.0]
    else:
        return fit


def analyse(path: str):
    video_name = path.split("/")[2]
    index, x_data, Y = data_open(path + "/radial_Avg.csv")
    x_data = np.array(x_data)
    data = []

    # Save all plots of I vs tau for each q
    for i, v in enumerate(zip(index, Y)):
        q, y = v
        y_data = np.array(y)
        y_max = np.max(y_data)
        fit = get_fit(func, x_data, y_data / y_max)
        fit = (fit[0] * y_max, fit[1], fit[2] * y_max)
        data.append(fit)
        # TODO: IS necessary ???
        plt.title(
            f"Plot of Intensity delta ({video_name}) for q={q} vs frame difference tau"
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
        if i % 10 == 0:
            print(f"{round(100 * i/len(index), 0)}% complete.")
        plt.savefig(f"{path}/I_vs_tau_for_q_{q}.png")
        plt.close()

    # # Save raw fit data
    with open(path + "/fit_data.csv", "w") as f:
        f.write("q, (a, b, c)\n")
        for q, (a, b, c) in zip(index, data):
            f.write(f"{q}, ({a}, {b}, {c})\n")

    # save log tau_c vs log q
    tau_c = np.log(np.array(list(map(lambda x: x[1], data))))
    q = np.log(np.array(index, dtype=np.float))
    plt.title(f"log(tau_c) vs log(q) for {video_name}")
    plt.ylabel("log(tau_c)")
    plt.xlabel("log(q)")
    plt.plot(q, tau_c)
    plt.savefig(f"{path}/tau_c_plot.png")
    plt.close()


# TODO: save csv with tau_c vs q for chosen video
if __name__ == "__main__":
    if os.path.isdir(argv[1]):
        files: List[str] = []
        for (dirpath, dirnames, filenames) in os.walk(argv[1]):
            files.extend(map(lambda s: f"./{dirpath}/{s}", filenames))
        files = list(filter(lambda s: s.find("radial_Avg.csv") != -1, files))
        directories = list(map(lambda s: s.replace("/radial_Avg.csv", ""), files))
        analyse(directories[0])
        for i, v in enumerate(directories):
            analyse(v)
            if i % 10 == 0:
                send_message(
                    secrets["twilio"],
                    f"Completed approximately {round(i * 100 / len(directories))}%.",
                )
