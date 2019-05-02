#!/usr/bin/env python3
import sys, os
from typing import List, Tuple, Optional
import sqlite3


import matplotlib.pyplot as plt


def data_open(
    file: str
) -> Tuple[List[float], Optional[List[float]], List[List[float]]]:
    with open(file) as f:
        raw_data = f.readlines()
    data: List[List[str]] = list(
        map(lambda s: s.replace("\n", "").split(",")[:-1], raw_data)
    )
    X = None
    index = list(map(float, data[0]))
    data = data[1:]
    cleaned = []
    for d in data:
        x = []
        y = []
        for v in d:
            s = v.replace("(", "").replace(")", "").split(" ")
            if X is None:
                x.append(float(s[0]))
            y.append(float(s[1]))
        if X is None:
            X = x
        cleaned.append(y)
    return (index, X, cleaned)


def plot():
    _, x, y = data_open(sys.argv[2])
    plt.plot(x, y[int(sys.argv[3])])
    plt.show()


def modify_db(database: str, folder: str, filename: str, prefix: str = "video"):
    conn = sqlite3.connect(database)
    files: List[str] = []
    for (dirpath, _, filenames) in os.walk(folder):
        files.extend(map(lambda s: f"./{dirpath}/{s}", filenames))
    files = list(
        filter(lambda s: s.find(filename) != -1 and s.find(".csv") != -1, files)
    )
    names = list(
        map(
            lambda s: f"{prefix}_"
            + os.path.basename(s).replace(
                filename, os.path.basename(os.path.dirname(s))
            ),
            files,
        )
    )
    create_table = (
        lambda table, tau: f"create table {table} (q float primary key, {tau})"
    )
    insert = lambda table, q, tau, I: (
        f"insert into {table} values (?, {', '.join(['?']*len(tau))})",
        map(lambda x: [x[0]] + [*x[1]], zip(q, I)),
    )
    for f, name in zip(files, names):
        q, tau_list, I_q_tau = data_open(f)
        if tau_list is not None:
            tau = ", ".join(map(lambda i: f"tau{int(i)} integer", tau_list))
            with conn:
                conn.execute(f"DROP TABLE IF EXISTS {name}")
            with conn:
                conn.execute(create_table(name, tau))
            with conn:
                conn.executemany(*insert(name, q, tau_list, I_q_tau))
            print(f"{name} added!")
        else:
            continue


if __name__ == "__main__":
    if len(sys.argv) == 4 and sys.argv[1] == "plot":
        plot()
    elif (
        len(sys.argv) == 3
        and sys.argv[1].find(".sqlite") != -1
        and os.path.exists(sys.argv[1])
    ):
        modify_db(sys.argv[1], sys.argv[2], "radial_Avg.csv")
    elif (
        len(sys.argv) == 5
        and sys.argv[1].find(".sqlite") != -1
        and os.path.exists(sys.argv[1])
        and os.path.exists(sys.argv[2])
    ):
        modify_db(sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4])
