# rust-hist

N-dimensional histograms in Rust.

# Development Instructions

Report bugs/issues or submit pull requests to <https://github.com/davehadley/rustplayground>.

Before committing any code, please install pre-commit hooks with:
```
source install-hook.sh
```

# Task List

- [x] Histogram 1D filling.
- [x] Histogram 1D retrieving.
- [x] Histogram 2D filling.
- [x] Histogram 2D retrieving.
- [x] Weighted Histogram 1D filling.
- [x] Weighted Histogram 1D retrieving.
- [x] Weighted Histogram 2D filling.
- [x] Weighted Histogram 2D retrieving.
- [x] Histogram 1D filling bin edges works as expected.
- [x] Histogram 2D filling bin edges works as expected.
- [ ] Make Histogram API match HashMap API.
- [ ] Histogram 1D underflow works as expected.
- [ ] Histogram 2D overflow works as expected.
- [ ] Histogram/axis iteration.
- [ ] Category axis.
- [ ] AnyD histogram.
- [ ] Growable axis.
- [ ] Re-binnable axis.
- [ ] Documentation.
- [ ] Make public on github.
- [ ] Release to create.io.
- [ ] numpy style slicing?

# Niggles

- [x] Don't make user have to bother with references/dereferences to Copy-able / primitives
- [ ] Harmonize indexes vs indices