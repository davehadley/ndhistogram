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
