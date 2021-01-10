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
- [x] Growable axis.
- [x] Noflow
- [x] Fix Histogram traits.
- [x] Fix Fill traits.
- [x] Make Histogram "Item" an associated type. --> hard to do without GAT
- [x] Move Axes to Axis module? --> decided not to do this
- [x] Move unit tests out to tests directory.
- [x] Remove grow.
- [x] Mega refactor
- [x] Remove unneeded dependencies
- [ ] Add tests for integer histogram 
- [ ] Check API and naming guidelines are met.
- [ ] Documentation.
- [ ] Make public on github.
- [ ] Release to create.io.
- [ ] AnyD histogram
- [ ] Implement Fill trait for weights tracker.
- [ ] Variable binning
- [ ] Integer binning
- [ ] numpy style slicing?
- [ ] All Histograms must implement Display (enforce in trait?)
- [ ] All Histograms must implement Eq (enforce in trait?)
- [x] All Histograms must implement Index (enforce in trait?) --> decided against as Index can't return option, only panic on bad index.
- [ ] Decide if we want a trait alias that is "CompleteHistogram : Histogram + Fill + MutableHistogram + FillWeight"
- [ ] Variable binning
- [ ] Uniform with mapping binning
- [ ] Implement grow for Uniform
- [ ] Make Uniform work for non-float
- [ ] Re-binnable axis.
- [ ] Reinstate grow
- [ ] Test each axis method.
- [ ] Test each histogram 1D method.
- [ ] Make Axes a struct to permit future optimisations.

# Niggles

- [x] Don't make user have to bother with references/dereferences to Copy-able / primitives
- [ ] Harmonize indexes vs indices
- [ ] Replace Box dyn iterators with static.
- [ ] Implement proper floating point behaviour on grow with float axis (binrange equality)