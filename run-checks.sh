#!/usr/bin/env bash

cargo check \
&& cargo fmt \
&& cargo clippy \
&& RUSTDOCFLAGS="-D warnings" cargo doc \
&& cargo publish --locked --dry-run