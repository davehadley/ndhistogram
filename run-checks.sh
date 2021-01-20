#!/usr/bin/env bash

cargo check \
&& cargo fmt \
&& cargo clippy \
&& RUSTDOCFLAGS="-D warnings" cargo doc \
&& cd ndhistogram && cargo publish --locked --dry-run