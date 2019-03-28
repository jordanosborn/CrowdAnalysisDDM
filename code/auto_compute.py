import json
import sys
import subprocess as sp
import os
import multiprocessing
from twilio.rest import Client
from typing import Any

with open("secrets.json") as f:
    secrets = json.loads(f.read())


def send_message(secrets: Any, body: str):
    account_sid = secrets["account_sid"]
    auth_token = secrets["auth_token"]
    client = Client(account_sid, auth_token)

    message = client.messages.create(
        body=body,
        from_='whatsapp:+14155238886',
        to=f'whatsapp:{secrets["phone_number"]}'
    )

    print(message.sid)


def run(command: str, video: str, capacity: int, radial_width: int):
    print(video)
    sp.call(["cargo", "run", "--release", command,
             str(capacity), str(radial_width), video])


if __name__ == "__main__":
    if len(sys.argv) == 4:
        files = []
        capacity, radial_width = int(sys.argv[2]), int(sys.argv[3])
        for (dirpath, dirnames, filenames) in os.walk(sys.argv[1]):
            files.extend(map(lambda s: f"./{dirpath}{s}", filenames))

        for index, video in enumerate(files):
            run("video-ddm", video, capacity, radial_width)
            if index % 10 == 0 and index != 0:
                send_message(
                    secrets["twilio"],
                    f"Have completed approximately {index * 100 / len(files)}%.")

    else:
        print(
            f"Arguments supplied are incorrect (_, directory, capacity, radial_width) - {sys.argv}")
