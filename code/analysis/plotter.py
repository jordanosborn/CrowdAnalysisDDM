import sqlite3
from matplotlib import pyplot as plt
import numpy as np
import sys

# TODO: add fit to this
if __name__ == "__main__":
    conn = sqlite3.connect("crowd.sqlite")
    if len(sys.argv) == 3 and sys.argv[1] == "search" and sys.argv[2] == "video":

        with conn:
            tables = list(
                filter(
                    lambda s: s.find(sys.argv[2]) != -1,
                    map(
                        lambda x: x[0],
                        conn.execute(
                            "select name from sqlite_master where type = 'table'"
                        ).fetchall(),
                    ),
                )
            )
        print("Found: ", "\n".join([str((i, t)) for i, t in enumerate(tables)]))
        table_id = int(input("Which would you like: "))  # nosec
        if 0 <= table_id < len(tables):
            while True:
                q = float(input("Which q would you like [1.5, 499.5, 1.0]? "))  # nosec
                if 1.5 <= q <= 499.5 and int(q) == q - 0.5:
                    table = tables[table_id]
                    print(f"You have selected {table} at q={q}")
                    with conn:
                        data = conn.execute(
                            f"select * from {table} where q={q}"
                        ).fetchone()[1:]
                    tau = [i + 1 for i in range(len(data))]

                    plt.title(
                        f"Plot of Intensity delta ({table.replace('video_', '')}) for q={q} vs frame difference tau"
                    )
                    plt.ylabel(f"I(q={q}, tau)")
                    plt.xlabel("tau")
                    plt.plot(tau, data)
                    plt.show()
                elif q < 0:
                    break
                else:
                    print("Invalid q selected!")
        else:
            print("Invalid table selected!")
    else:
        print("Invalid args")
