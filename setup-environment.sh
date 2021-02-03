#!/usr/bin/bash

# Activate python (needed for pre-commit hooks) 
if [ ! -d "venv" ]
then
    python3 -m venv venv \
    && source venv/bin/activate \
    && python3 -m pip install pre-commit isort mypy black flake8;
fi
source venv/bin/activate

# Activate rust
if [ ! -f "$HOME/.cargo/env" ]
then
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
    source $HOME/.cargo/env
    # install cargo commands
    cargo install cargo-expand cargo-sync-readme
fi
source $HOME/.cargo/env
