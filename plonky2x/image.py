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
               .apt_install(["git", "curl", "pkg-config", "libssl-dev"])
               .pip_install(["gitpython", "boto3", "requests"])
               .env({"SHELL": "/bin/bash"})
               .run_commands([
                "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y",
                "ln -s /root/.cargo/bin/cargo /usr/bin/cargo",
                "ln -s /root/.cargo/bin/rustup /usr/bin/rustup"
               ])
               )

base_image = test_image


@stub.function(image=base_image, cpu=4, cloud="aws", timeout=60*50, mounts=[modal.Mount.from_local_dir(".", remote_path="/root/")])
def run():
    mv = subprocess.run(
        f'rustup override set nightly && cargo run --bin mapreduce_example --release', shell=True)
    print(mv)
    return "Hello World"

@stub.local_entrypoint()
def main():
    output = run.remote()
    print("hello world", output)