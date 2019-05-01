#!/usr/bin/env python3
import json, math
import sys
import subprocess as sp  # nosec
import os
from twilio.rest import Client
from typing import Union, Any, List, Iterable, Optional
from itertools import product


def get_allowed_dimension(
    tiling_min: int, tiling_max: int, tiling_size_count: Optional[int]
) -> List[int]:
    power2 = math.ceil(math.log2(tiling_max))
    power3 = math.ceil(math.log(tiling_max, 3))
    power5 = math.ceil(math.log(tiling_max, 5))
    box_range_pre = product(range(power2 + 1), range(power3 + 1), range(power5 + 1))
    box_range: List[int] = list(
        filter(
            lambda x: tiling_min <= x <= tiling_max,
            map(lambda x: int(2 ** x[0] * 3 ** x[1] * 5 ** x[2]), box_range_pre),
        )
    )
    box_range.sort()
    if tiling_size_count is not None:
        length = len(box_range)
        if not (tiling_size_count <= length):
            tiling_size_count = length

        new_vec = []

        for i in range(tiling_size_count - 1):
            new_vec.append(box_range[int(i * length / math.ceil(tiling_size_count))])
        new_vec.append(box_range[length - 1])
        return new_vec
    else:
        return box_range


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


def run(command: str, video: str, capacity: str, radial_width: str):
    print(video)
    if command == "video-multi-ddm":
        # TODO: run on range of box sizes to prevent resource starvation
        # size_range = get_allowed_dimension(16, 1024, 16)
        sp.call(
            [
                "cargo",
                "run",
                "--release",
                command,
                str(capacity),
                str(radial_width),
                str(64),
                str(1024),
                str(16),
                video,
            ]
        )
    else:
        sp.call(
            [
                "cargo",
                "run",
                "--release",
                command,
                str(capacity),
                str(radial_width),
                video,
            ]
        )


def upload():
    sp.call(["git", "add", "."])
    sp.call(["git", "commit", "-m", '"added more data"'])
    sp.call(["git", "pull", "--rebase"])
    sp.call(["git", "push"])


def contains_any(string: str, to_check: List[str]) -> bool:
    return any(map(lambda x: string.find(x) != -1, to_check))


def incomplete_filter(files: List[str], directory: str) -> Iterable[str]:
    completed_videos = []
    for (_, dirnames, _) in os.walk(directory):
        completed_videos.extend(dirnames)
    return filter(lambda x: not contains_any(x, completed_videos), files)


def filter_non_videos(files: Union[Iterable[str], List[str]]) -> Iterable[str]:
    video_filetypes = [".avi", ".mp4", ".m4v"]
    return filter(lambda s: contains_any(s, video_filetypes), files)


def retranspose(files: List[str]):
    for i, f in enumerate(files):
        file_path = f.replace("./", "").replace("results", "results-transposed")
        try:
            os.mkdir("/".join(file_path.split("/")[0:-1]))
        except FileExistsError:
            pass
        else:
            sp.call(
                [
                    "cargo",
                    "run",
                    "--release",
                    "retranspose",
                    f.replace("./", ""),
                    "output.csv",
                ]
            )
            sp.call(["mv", "output.csv", file_path])
            print(f"Completed {(i+1) * 100 / len(files)}%.")


def add_to_db(
    db: str, folder: str, filename: str = "radial_Avg.csv", prefix: str = "video"
):
    sp.call(["python3", "analysis/data_clean.py", db, folder, filename, prefix])


if __name__ == "__main__":
    files: List[str] = []
    if (
        len(sys.argv) == 3
        and sys.argv[1] in ["video-multi-ddm", "video-ddm"]
        and os.path.isdir(sys.argv[2])
    ):
        sys.argv = sys.argv + ["80", "1"]
    if (
        len(sys.argv) == 5
        and sys.argv[1] in ["video-multi-ddm", "video-ddm"]
        and os.path.isdir(sys.argv[2])
    ):
        files = []
        capacity, radial_width = sys.argv[3], sys.argv[4]
        for (dirpath, dirnames, filenames) in os.walk(sys.argv[2]):
            files.extend(
                map(
                    lambda s: f"./{dirpath}{s}",
                    filter(
                        lambda s: s.split(".")[-1] in ["avi", "mp4", "m4v"], filenames
                    ),
                )
            )
        files_filtered = incomplete_filter(files, "./results-multiDDM")
        files_filtered = list(filter_non_videos(files_filtered))
        print(f"{len(files_filtered)}/{len(files)} left to analyse.")
        for index, video in enumerate(files_filtered):
            run(sys.argv[1], video, capacity, radial_width)
            if index % 5 == 0 and index != 0:
                send_message(
                    secrets["twilio"],
                    f"Have completed approximately {round((index + len(files) - len(files_filtered)) * 100 / len(files), 2)}%.",
                )
                upload()

        if sys.argv[1] == "video-multi-ddm":
            add_to_db(
                "crowd.sqlite", "results-multiDDM", "data_boxsize", "video-multi-ddm"
            )
        else:
            print("Producing retranspose")
            files = []
            for (dirpath, dirnames, filenames) in os.walk("./results"):
                files.extend(
                    filter(
                        lambda s: s.find("radial_Avg.csv") != -1,
                        map(lambda s: f"./{dirpath}/{s}", filenames),
                    )
                )
            retranspose(files)
            add_to_db("crowd.sqlite", "results-transposed")
        upload()
    elif len(sys.argv) == 3 and sys.argv[1] == "fit" and os.path.isdir(sys.argv[2]):
        sp.call(["python3", "./analysis/analyse.py", *sys.argv[2:]])
        upload()
    elif len(sys.argv) == 2 and sys.argv[1] == "plot":
        sp.call(["python3", "./analysis/plotter.py", "search", "video"])
    elif len(sys.argv) == 5 and sys.argv[1] == "resize" and os.path.isdir(sys.argv[2]):
        files = []
        for (dirpath, dirnames, filenames) in os.walk(sys.argv[2]):
            files.extend(
                map(
                    lambda f: os.path.join(dirpath, f),
                    filter(
                        lambda f: any(
                            [f.find(ext) != -1 for ext in ["avi", "mp4", "m4v"]]
                        ),
                        filenames,
                    ),
                )
            )
        out_dir = f"{os.path.dirname(sys.argv[2])}_resized"
        output = list(map(lambda s: os.path.join(out_dir, os.path.basename(s)), files))
        print("Starting conversion")
        if not os.path.isdir(out_dir):
            os.mkdir(out_dir)
        for filename, out in zip(files, output):
            sp.call(
                ["python3", "./analysis/video_resizer.py", filename, out, *sys.argv[3:]]
            )
    elif len(sys.argv) == 6 and sys.argv[1] == "resize":
        sp.call(["python3", "./analysis/video_resizer.py", *sys.argv[2:]])
    elif (
        len(sys.argv) == 3
        and sys.argv[1] == "retranspose"
        and os.path.isdir(sys.argv[2])
    ):
        files = []
        for (dirpath, dirnames, filenames) in os.walk(sys.argv[2]):
            files.extend(
                filter(
                    lambda s: s.find("radial_Avg.csv") != -1,
                    map(lambda s: f"./{dirpath}/{s}", filenames),
                )
            )
        retranspose(files)
        upload()
    elif len(sys.argv) == 6 and sys.argv[1] == "add_to_db":
        add_to_db(sys.argv[2], sys.argv[3], sys.argv[4], sys.argv[5])
    else:
        print(
            f"Arguments supplied are incorrect (_, directory, capacity, radial_width) - {sys.argv}"
        )
