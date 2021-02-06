#!/usr/bin/env python3

from pathlib import Path
from subprocess import run

packagedirectory = Path(__file__).parents[1]
assert packagedirectory.exists()

run(["pre-commit", "install"], cwd=packagedirectory)
