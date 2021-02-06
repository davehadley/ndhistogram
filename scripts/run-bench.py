#!/usr/bin/env python3

from pathlib import Path
from subprocess import run

packagedirectory = Path(__file__).parents[1]
assert packagedirectory.exists()

commithash = run(
    ["git", "rev-parse", "--short", "HEAD"],
    cwd=packagedirectory,
    capture_output=True,
    text=True,
).stdout.strip("\n")
run(["cargo", "bench", "--", "--save-baseline", commithash], cwd=packagedirectory)
