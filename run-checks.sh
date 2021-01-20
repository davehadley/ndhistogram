#!/usr/bin/env bash

cargo check \
&& cargo fmt \
&& cargo clippy -- -W clippy::pedantic \
&& RUSTDOCFLAGS="-D warnings" cargo doc \
&& cd ndhistogram && cargo publish --locked --dry-run
