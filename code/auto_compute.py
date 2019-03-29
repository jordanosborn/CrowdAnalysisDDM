import json
import sys
import subprocess as sp
import os
import multiprocessing
from twilio.rest import Client
from typing import Any, List, Iterable

# TODO: retranspose everything

with open("secrets.json") as f:
    secrets = json.loads(f.read())


def send_message(secrets: Any, body: str):
    account_sid = secrets["account_sid"]
    auth_token = secrets["auth_token"]
    client = Client(account_sid, auth_token)

    message = client.messages.create(
        body=body,
        from_=f'{secrets["twilio_number"]}',
        to=f'{secrets["phone_number"]}'
    )
    print(
        f'Sent message to {secrets["phone_number"]} message_ID = {message.sid}')


def run(command: str, video: str, capacity: int, radial_width: int):
    print(video)
    sp.call(["cargo", "run", "--release", command,
             str(capacity), str(radial_width), video])


def upload():
    sp.call(["git", "add", "."])
    sp.call(["git", "commit", "-m", "\"added more data\""])
    sp.call(["git", "pull", "--rebase"])
    sp.call(["git", "push"])


def contains_any(string: str, to_check: List[str]) -> bool:
    return any(map(lambda x: string.find(x) != -1, to_check))


def incomplete_filter(files: List[str]) -> Iterable[str]:
    completed_videos = []
    for (dirpath, dirnames, filenames) in os.walk("./results"):
        completed_videos.extend(dirnames)
    return filter(lambda x: not contains_any(x, completed_videos), files)


if __name__ == "__main__":
    if len(sys.argv) == 3 and sys.argv[1] in ["video-multi-ddm", "video-ddm"] and os.path.isdir(sys.argv[2]):
        sys.argv = sys.argv + [80, 1]
    if len(sys.argv) == 5 and sys.argv[1] in ["video-multi-ddm", "video-ddm"] and os.path.isdir(sys.argv[2]):
        files = []
        capacity, radial_width = int(sys.argv[3]), int(sys.argv[4])
        for (dirpath, dirnames, filenames) in os.walk(sys.argv[2]):
            files.extend(map(lambda s: f"./{dirpath}{s}", filenames))
        files_filtered = list(incomplete_filter(files))
        print(f"{len(files_filtered)}/{len(files)} left to analyse.")
        for index, video in enumerate(files_filtered):
            run(sys.argv[1], video, capacity, radial_width)
            if index % 3 == 0 and index != 0:
                send_message(
                    secrets["twilio"],
                    f"Have completed approximately {(index + len(files) -len(files_filtered)) * 100 / len(files)}%.")
                upload()
    else:
        send_message(secrets["twilio"], "hello there")
        print(
            f"Arguments supplied are incorrect (_, directory, capacity, radial_width) - {sys.argv}")
