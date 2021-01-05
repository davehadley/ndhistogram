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
- [x] Make Histogram API match HashMap API.
- [x] Histogram 1D underflow works as expected. --> make no distinction (except in BinRange enum), forces user to handle underflow/overflow
- [x] Histogram 2D overflow works as expected.
- [x] Test each axis method.
- [x] Test each histogram 1D method.
- [x] Category axis.
- [ ] Growable axis.
- [ ] Re-binnable axis.
- [ ] Test each axis method.
- [ ] Test each histogram 1D method.
- [ ] AnyD histogram.
- [ ] Check API and naming guidelines are met.
- [ ] Documentation.
- [ ] Make public on github.
- [ ] Release to create.io.
- [ ] numpy style slicing?
- [ ] All Histograms must implement Display (enforce in trait?)
- [ ] All Histograms must implement Eq (enforce in trait?)
- [x] All Histograms must implement Index (enforce in trait?) --> decided against as Index can't return option, only panic on bad index.
- [ ] Decide if we want a trait alias that is "CompleteHistogram : Histogram + Fill + MutableHistogram + FillWeight"

# Niggles

- [x] Don't make user have to bother with references/dereferences to Copy-able / primitives
- [ ] Harmonize indexes vs indices
- [ ] Replace Box dyn iterators with static.