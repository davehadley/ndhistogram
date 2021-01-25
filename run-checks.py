#!/usr/bin/env python3

from os import environ
from subprocess import run


def env(**kwargs):
    return {**environ, **kwargs}


run(["cargo", "check"], check=True)
run(["cargo", "fmt"], check=True)
run(["cargo", "clippy"], check=True)
run(
    [
        "cargo",
        "doc",
    ],
    env=env(RUSTDOCFLAGS="-D warnings"),
    check=True,
)
run(
    [
        "cargo",
        "sync-readme",
        "--check",
    ],
    cwd="ndhistogram",
    check=True,
)
run(["cargo", "publish", "--locked", "--dry-run"], cwd="ndhistogram", check=True)
