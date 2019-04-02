#!/usr/bin/env python3
import json
import sys
import subprocess as sp  # nosec
import os
from twilio.rest import Client
from typing import Union, Any, List, Iterable


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
    sp.call(
        ["cargo", "run", "--release", command, str(capacity), str(radial_width), video]
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


if __name__ == "__main__":
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
        files: List[str] = []
        capacity, radial_width = sys.argv[3], sys.argv[4]
        for (dirpath, dirnames, filenames) in os.walk(sys.argv[2]):
            files.extend(map(lambda s: f"./{dirpath}{s}", filenames))
        files_filtered = incomplete_filter(files, "./results")
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
    elif (
        len(sys.argv) == 3
        and sys.argv[1] == "retranspose"
        and os.path.isdir(sys.argv[2])
    ):
        files: List[str] = []
        for (dirpath, dirnames, filenames) in os.walk(sys.argv[2]):
            files.extend(
                filter(
                    lambda s: s.find("radial_Avg.csv") != -1,
                    map(lambda s: f"./{dirpath}/{s}", filenames),
                )
            )
        retranspose(files)
    else:
        print(
            f"Arguments supplied are incorrect (_, directory, capacity, radial_width) - {sys.argv}"
        )
