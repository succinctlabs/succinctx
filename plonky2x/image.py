import glob
import json
import os
import stat
import subprocess
from typing import List
import tempfile

import boto3
import requests
import time

import modal
from modal import Image, Secret, Stub, Mount, Volume, web_endpoint
from pydantic import BaseModel

stub = Stub("mapreduce")

test_image = (Image
              .debian_slim()
              .apt_install(["git", "curl"])
              .pip_install(["gitpython", "boto3", "requests"])
              )

base_image = test_image.apt_install(
    "build-essential libgmp-dev libsodium-dev nasm nlohmann-json3-dev libc6 gcc-4.9 libstdc++6 glibc-source".split(" "))


@stub.function(image=base_image, cpu=4, cloud="aws", timeout=60*50, mounts=[modal.Mount.from_local_dir("./build", remote_path="/root/build")])
def run():
    mv = subprocess.run(
        f'chmod +x /root/build/mapreduce_example && /root/build/mapreduce_example', shell=True)
    print(mv)
    return "Hello World"

@stub.local_entrypoint()
def main():
    output = run.remote()
    print("hello world", output)