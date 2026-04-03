# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.12.0](https://github.com/davehadley/ndhistogram/compare/v0.11.0...v0.12.0) - 2026-04-03

### Added

- [**breaking**] allow user provided hasher for HashHistogram ([#43](https://github.com/davehadley/ndhistogram/pull/43))

## [0.11.0](https://github.com/davehadley/ndhistogram/compare/v0.10.1...v0.11.0) - 2026-03-31

### Added

- implement from_map for HashHistogram
- implement from_vec for VecHistogram
- conversion from HashHistogram into HashMap
- implement From/Into for Vec/VecHistogram
- implement as_slice for VecHistogram
- implement as_vec for VecHistogram

### Other

- fix HTMLrender error in docs
- remove html_root_url as it is no longer recommended
- *(deps)* [**breaking**] upgrade thiserror to 2.0
- update latest release in docs

## [0.10.1](https://github.com/davehadley/ndhistogram/compare/v0.10.0...v0.10.1) - 2026-03-29

### Other

- update readme
- update msrv in CI
- run cargo-sync-readme
- add commitlint and dprint to CI
- auto format toml
- cargo fmt
- relax dependency requirements, update msrv
- fix clippy lints
- Fix some compiler warnings in tests.
- Update dependencies.

# 0.10.0 (2022-11-14)

- Fix [issue 33](https://github.com/davehadley/ndhistogram/issues/33): Filling a histogram where the axis value is NaN not longer panics. NaN is mapped to the overflow bin where one exists or to no bin on axes without overflow bins.
- Most cases where the library could panic have been replaced with functions that return Result instead.
- Fix new clippy and compiler warnings.
- Minimum supported rust version is increased to 1.63.0.

# 0.9.0 (2023-05-19)

- Make serde an optional dependency. This is a breaking change. If you were using serde with this crate's types you will have to enable the "serde" feature flag in your `Cargo/toml`.
- Implement rayon support for parallel iteration over histogram bins. For example see: [VecHistogram::par_iter](https://docs.rs/ndhistogram/0.9.0/struct.VecHistogram.html#method.par_iter) and [HashHistogram::par_iter](https://docs.rs/ndhistogram/0.9.0/struct.HashHistogram.html#method.par_iter). These features are enabled with the "rayon" crate feature flag.
- Minimum supported rust version is now specified.
- Improvements to continuous integration.
- Implement `FillWith` trait for `Sum` value type (from [@rfoxkendo](https://github.com/rfoxkendo)).
- Fix some broken documentation links.

# 0.8.0 (2022-08-04)

- Improved histogram merging. [VecHistogram](https://docs.rs/ndhistogram/0.8.0/ndhistogram/struct.VecHistogram.html) and [HashHistogram](https://docs.rs/ndhistogram/0.8.0/ndhistogram/struct.HashHistogram.html) now implement the [AddAssign] trait and implement the [Add] trait on owned values (where merging can be done without copying).
- Improvements to documentation.

# 0.7.0 (2022-06-06)

- New [UniformCyclic](https://docs.rs/ndhistogram/0.7.0/ndhistogram/axis/struct.UniformCyclic.html) and [VariableCyclic](https://docs.rs/ndhistogram/0.7.0/ndhistogram/axis/struct.VariableCyclic.html) axis types (from [@jacg](https://github.com/jacg)).

# 0.6.3 (2022-04-07)

- `Uniform::with_step_size` now panics when given a step size equal to zero (from [@jacg](https://github.com/jacg)).
- Documentation and CI improvements from [@jacg](https://github.com/jacg).
- Fix compiler/clippy warnings when building with the latest stable version (clippy 0.1.59, rustc 1.59.0).

# 0.6.2 (2021-10-16)

- Fix for compiler/clippy warnings when building with the latest stable version (clippy 0.1.55, rustc 1.55.0).
- Include pre-commit hooks in continuous integration.

# 0.6.1 (2021-06-27)

- Fix for compiler/clippy warnings when building with the latest stable version (clippy 0.1.53, rustc 1.53.0).
- Improved documentation.

# 0.6.0 (2021-02-06)

- Histogram filling performance improvements (uniform axis histograms ~80% faster, variables axis histograms ~30% faster).
- Added criterion benchmarks.

# 0.5.0 (2021-01-24)

- Implement integer binning for Uniform and UniformNoFlow histograms.
- Improve documentation.
