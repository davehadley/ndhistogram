#!/usr/bin/env bash

cargo bench -- --save-baseline $(git rev-parse --short HEAD)
