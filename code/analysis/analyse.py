#!/usr/bin/env python3
from data_clean import data_open
from sys import argv
from matplotlib import pyplot as plt
import numpy as np
from scipy.optimize import curve_fit
import os
from twilio.rest import Client
from typing import Any, List, Callable, Dict, Tuple
import json
from collections import OrderedDict

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


def get_fit(f, x, y, bounds):
    try:
        fit, _ = curve_fit(func, x, y, bounds=(bounds[0], bounds[1]))
    except RuntimeError:
        return [np.nan] * len(bounds[0])
    else:
        return fit


def analyse(
    path: str,
    function: Callable,
    bounds_dict: Dict[str, Tuple[Any, ...]],
    plot_param: int,
    function_string="a * np.exp(-x / b) + c",
):
    video_name = list(filter(lambda s: s != "", path.split("/")))[-1]
    index, x_data, Y = data_open(path + "/radial_Avg.csv")
    x_data = np.array(x_data)
    data = []
    parameters = list(bounds_dict.keys())
    bounds = (
        [v[0] for _, v in bounds_dict.items()],
        [v[1] for _, v in bounds_dict.items()],
    )
    # Save all plots of I vs tau for each q
    for i, v in enumerate(zip(index, Y)):
        q, y = v
        y_data = np.array(y)
        fit = get_fit(function, x_data, y_data, bounds)
        # fit = (fit[0] * y_max, fit[1], fit[2] * y_max)
        data.append(fit)
        plt.title(
            f"Plot of Intensity delta ({video_name}) for q={q} vs frame difference tau"
        )
        plt.ylabel(f"I(q={q}, tau)")
        plt.xlabel("tau")
        plt.plot(x_data, y_data, label="data")
        plt.plot(
            x_data,
            func(x_data, *fit),
            label=f"fit f(tau) = {function_string.replace('np.', '')} with {', '.join(map(lambda x: f'{x[0]}={x[1]}', zip(parameters, map(lambda s: round(s, 2), fit))))}",
        )
        plt.legend(loc="lower right")
        if i % 10 == 0:
            print(f"{round(100 * i/len(index), 0)}% complete.")
        plt.savefig(f"{path}/I_vs_tau_for_q_{q}.png")
        plt.close()
    print(f"100% complete.")
    # # Save raw fit data
    with open(path + "/fit_data.csv", "w") as f:
        f.write(f"q, ({', '.join(parameters)})\n")
        for q, d in zip(index, data):
            params = f"({','.join(map(str, d))})"
            f.write(f"{q}, {params}\n")

    # save log tau_c vs log q
    tau_c = np.log(np.array(list(map(lambda x: x[plot_param], data))))
    q = np.log(np.array(index, dtype=np.float))
    plt.title(f"log(tau_c) vs log(q) for {video_name}")
    plt.ylabel("log(tau_c)")
    plt.xlabel("log(q)")
    plt.plot(q, tau_c)
    plt.savefig(f"{path}/tau_c_plot.png")
    plt.close()


if __name__ == "__main__":
    if os.path.isdir(argv[1]):
        files: List[str] = []
        for (dirpath, dirnames, filenames) in os.walk(argv[1]):
            files.extend(map(lambda s: f"./{dirpath}/{s}", filenames))
        files = list(filter(lambda s: s.find("radial_Avg.csv") != -1, files))
        directories = list(map(lambda s: s.replace("/radial_Avg.csv", ""), files))
        analyse(
            directories[0],
            func,
            OrderedDict(
                {"a": (-np.inf, np.inf), "b": (0, np.inf), "c": (-np.inf, np.inf)}
            ),
            1,
        )
        for i, v in enumerate(directories):
            analyse(
                v,
                func,
                OrderedDict(
                    {"a": (-np.inf, np.inf), "b": (0, np.inf), "c": (-np.inf, np.inf)}
                ),
                1,
            )
            if i % 10 == 0:
                send_message(
                    secrets["twilio"],
                    f"Completed approximately {round(i * 100 / len(directories))}%.",
                )
    elif os.path.isfile(argv[1]) and argv[1].find("radial_Avg.csv") != -1:
        analyse(
            argv[1].replace("radial_Avg.csv", ""),
            func,
            OrderedDict(
                {"a": (-np.inf, np.inf), "b": (0, np.inf), "c": (-np.inf, np.inf)}
            ),
            1,
        )
    elif (
        os.path.isfile(argv[1])
        and argv[1].find("radial_Avg.csv") != -1
        and argv[2] == "custom"
    ):
        print("Errors are not checked!")
        params_str = input(  # nosec
            "Comma spaced parameter list with range e.g.  A(0: np.inf)? "
        )
        params = params_str.replace(" ", "").replace("\t", "").split(",")
        bounds: Dict[str, Tuple[Any, ...]] = OrderedDict()
        for p in params:
            name, values = p.replace(")", "").split("(")
            bounds[name] = tuple(map(eval, values.split(":")))
        independent_vars = input(  # nosec
            "Please enter comma separated list of independent variable names? "
        ).split(",")
        independent_vars = list(
            filter(
                lambda s: s != "",
                map(lambda s: s.replace(" ", "").replace("\t", ""), independent_vars),
            )
        )
        function_string = input(  # nosec
            "Please enter function to fit to using params? "
        )
        plot_param = int(
            input("Please enter the index (starting 0) of the final plot? ")  # nosec
        )
        print(bounds, "\n", f"f({', '.join(independent_vars)}) = {function_string}")
        if input("Are these correct (y/n)? ").strip() == "y":  # nosec
            function = eval(
                f"lambda {','.join(independent_vars)}, {','.join(bounds.keys())}: {function_string}"
            )
            analyse(
                argv[1].replace("/radial_Avg.csv", ""),
                function,
                bounds,
                plot_param,
                function_string,
            )
        else:
            print("Try again!")
