#!/usr/bin/env python3

from os import environ
from pathlib import Path
from subprocess import run

packagedirectory = Path(__file__).parents[1]
assert packagedirectory.exists()


def env(**kwargs):
    return {**environ, **kwargs}


run(["cargo", "check"], check=True, cwd=packagedirectory)
run(["cargo", "fmt"], check=True, cwd=packagedirectory)
run(["cargo", "clippy"], check=True, cwd=packagedirectory)
run(
    [
        "cargo",
        "doc",
    ],
    env=env(RUSTDOCFLAGS="-D warnings"),
    check=True,
    cwd=packagedirectory,
)
if run(["cargo", "sync-readme", "-V"], cwd=packagedirectory).returncode != 0:
    run(["cargo", "install", "cargo-sync-readme"], cwd=packagedirectory)
run(
    [
        "cargo",
        "sync-readme",
        "--check",
    ],
    cwd=packagedirectory / "ndhistogram",
    check=True,
)
run(
    ["cargo", "publish", "--locked", "--dry-run"],
    cwd=packagedirectory / "ndhistogram",
    check=True,
)
