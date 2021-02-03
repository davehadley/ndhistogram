# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.155.1/containers/rust/.devcontainer/base.Dockerfile


FROM mcr.microsoft.com/vscode/devcontainers/rust:1
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends python3-pip
RUN python3 -m pip install pre-commit isort mypy black flake8
