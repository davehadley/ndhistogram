# ndhistogram : N-dimensional histograms for Rust

[![Main Build status](https://img.shields.io/github/actions/workflow/status/davehadley/ndhistogram/ci.yml?branch=main&label=main)](https://github.com/davehadley/ndhistogram)
[![Develop status](https://img.shields.io/github/actions/workflow/status/davehadley/ndhistogram/ci.yml?branch=develop&label=develop)](https://github.com/davehadley/ndhistogram)
[![Documentation](https://docs.rs/ndhistogram/badge.svg)](https://docs.rs/ndhistogram/)
[![Codecov](https://codecov.io/github/davehadley/ndhistogram/coverage.svg?branch=main)](https://codecov.io/gh/davehadley/ndhistogram)
[![Dependency status](https://deps.rs/repo/github/davehadley/ndhistogram/status.svg)](https://deps.rs/repo/github/davehadley/ndhistogram)
[![Crate](https://img.shields.io/crates/v/ndhistogram.svg)](https://crates.io/crates/ndhistogram)
[![License](https://img.shields.io/crates/l/ndhistogram)](https://crates.io/crates/ndhistogram)
[![Last commit](https://img.shields.io/github/last-commit/davehadley/ndhistogram/develop)](https://github.com/davehadley/ndhistogram)
[![Last release](https://img.shields.io/github/release-date/davehadley/ndhistogram)](https://crates.io/crates/ndhistogram)


# Usage

For usage instructions see:
- [crates.io package](https://crates.io/crates/ndhistogram)
- [documentation](https://docs.rs/ndhistogram)
- [ndhistogram readme](ndhistogram/README.md)

# Development Instructions

If you wish to make a contribution to this repository please read the 
[contribution guide](https://github.com/davehadley/ndhistogram/blob/develop/ndhistogram/README.md#how-to-contribute).

For development you will need the [rust toolchain installed](https://rustup.rs/)
and [pre-commit](https://pre-commit.com/) to run pre-commit hooks.
Some helper scripts are included to help you setup a working development
environment.

Setup the development environment with:
```
source setup-environment.sh
```
or you may use the 
[development container](https://code.visualstudio.com/docs/remote/containers)
inside `.devcontainer/Dockerfile`.

Before committing any code, please install pre-commit hooks with:
```
./scripts/install-hook.py
```

Some additional checks may be run with:
```
./scripts/run-checks.py
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.