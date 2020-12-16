#!/usr/bin/bash

# Activate python (needed for pre-commit hooks) 
if [ ! -d "venv" ]
then
    python3 -m venv venv \
    && source venv/bin/activate \
    && python -m pip install pre-commit isort mypy black flake8;
fi
source venv/bin/activate

# Activate rust
if [ ! -f "$HOME/.cargo/env" ]
then
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
fi
source $HOME/.cargo/env
