# ndhistogram : multi-dimensional histogramming for Rust

[![Main Build status](https://img.shields.io/github/actions/workflow/status/davehadley/ndhistogram/ci.yml?branch=main&label=main)](https://github.com/davehadley/ndhistogram)
[![Develop status](https://img.shields.io/github/actions/workflow/status/davehadley/ndhistogram/ci.yml?branch=develop&label=develop)](https://github.com/davehadley/ndhistogram)
[![Documentation](https://docs.rs/ndhistogram/badge.svg)](https://docs.rs/ndhistogram/)
[![Codecov](https://codecov.io/github/davehadley/ndhistogram/coverage.svg?branch=main)](https://codecov.io/gh/davehadley/ndhistogram)
[![Dependency status](https://deps.rs/repo/github/davehadley/ndhistogram/status.svg)](https://deps.rs/repo/github/davehadley/ndhistogram)
[![Crate](https://img.shields.io/crates/v/ndhistogram.svg)](https://crates.io/crates/ndhistogram)
[![License](https://img.shields.io/crates/l/ndhistogram)](https://crates.io/crates/ndhistogram)
[![Last commit](https://img.shields.io/github/last-commit/davehadley/ndhistogram/develop)](https://github.com/davehadley/ndhistogram)
[![Last release](https://img.shields.io/github/release-date/davehadley/ndhistogram)](https://crates.io/crates/ndhistogram)

<!-- cargo-sync-readme start -->

ndhistogram implements multi-dimensional histograms for Rust.


This library aims to provide a similar feature set to the C++ library
[boost-histogram](https://www.boost.org/doc/libs/1_75_0/libs/histogram)
but with an idomatic pure-Rust implementation.

Features include:
- Histograms with any number of dimensions from 1 up to 21 dimensions.
- Continuous (eg represented by a floating point number) and discrete axis (eg a category represented by a string value or enum) types that are composable (eg you may mix discrete and continuous axes).
- Flexible bin values including any primitive number type, or a user-defined type.
- Unweighted and weighted filling of histograms.
- Flexible, user-definable axis types.
- Sparse histograms to reduce the memory footprint of high bin count, mostly empty, histograms.

## Table of Contents

1. [Usage](#usage)
2. [Quick-start](#quick-start)
3. [Overview](#overview)
   1. [Histogram Implementations](#histogram-implementations)
   2. [Axis Implementations](#axis-implementations)
   3. [Histogram Bin Values](#histogram-bin-values)
4. [How to Guide](#how-to-guide)
   1. [Customize the Bin Value Type](#customize-the-bin-value-type)
   2. [Create and Use a 2D Histogram](#create-and-use-a-2d-histogram)
   3. [Create a Histogram with a Discrete Axis](#create-a-histogram-with-a-discrete-axis)
   4. [Create a Histogram with Variable Sized Bins](#create-a-histogram-with-variable-sized-bins)
   5. [Create a Histogram with a Periodic or Cyclic Axis](#create-a-histogram-with-a-periodic-or-cyclic-axis)
   6. [Create a Sparse Histogram](#create-a-sparse-histogram)
   7. [Merge Histograms](#merge-histograms)
   8. [Iterate over Histogram Bins in Parallel](#iterate-over-histogram-bins-in-parallel)
5. [Crate Feature Flags](#crate-feature-flags)
6. [How to contribute](#how-to-contribute)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
ndhistogram = "0.10.0"
```

See the [change log](https://github.com/davehadley/ndhistogram/blob/main/ndhistogram/CHANGELOG.md)
for differences between releases.
Please report any bugs in the [issues tracker](https://github.com/davehadley/ndhistogram/issues).

## Quick-start

```rust
use ndhistogram::{Histogram, ndhistogram, axis::Uniform};

// create a 1D histogram with 10 equally sized bins between -5 and 5
let mut hist = ndhistogram!(Uniform::new(10, -5.0, 5.0)?);
// fill this histogram with a single value
hist.fill(&1.0);
// fill this histogram with weights
hist.fill_with(&2.0, 4.0);
// read the histogram values
let x1 = hist.value(&1.0);
let also_x1 = hist.value_at_index(7);
assert_eq!(x1, also_x1);
// iterate the histogram values
for item in hist.iter() {
    println!("{}, {}, {}", item.index, item.bin, item.value)
}
// print the histogram to stdout
println!("{}", hist);
```

## Overview

A [Histogram] is composed of two components:
- The [Axes] which is a set of [Axis](axis::Axis) corresponding to each dimension of the histogram.
  The [Axes] and [Axis](axis::Axis) define the binning of the histogram and are responsible for mapping from coordinate space (eg \[x,y,z\]) to an integer bin number.
- The histogram bin value storage. Valid bin value types include any integer and floating number type as well as user defined types that implement [Fill], [FillWith] or [FillWithWeighted].

### Histogram Implementations

- [VecHistogram]: bin values are stored in a [Vec].
  Created with the [ndhistogram] macro.
  This is the recommended implementation for most use cases.
  However, as memory is allocated even for empty bins,
  this may not be practical for very high dimension histograms.
- [HashHistogram]: bin values are stored in a [HashMap](std::collections::HashMap).
  Created with the [sparsehistogram] macro.
  Useful for high dimension, mostly empty, histograms as empty bins
  take up no memory.

Alternative implementations are possible by implementing the [Histogram] trait.

### Axis Implementations

- [Uniform](axis::Uniform)/[UniformNoFlow](axis::UniformNoFlow): equally sized bins in a some range with optional underflow/overflow bins.
- [Variable](axis::Variable)/[VariableNoFlow](axis::VariableNoFlow): variable sized bins with optional underflow/overflow bins.
- [UniformCyclic](axis::UniformCyclic)/[VariableCyclic](axis::VariableCyclic): cyclic or periodic versions of the Uniform and Variable axes.
- [Category](axis::Category)/[CategoryNoFlow](axis::CategoryNoFlow): a finite set of discrete values with optional overflow bin.

User defined axes types are possible by implementing the [Axis](axis::Axis) trait.

### Histogram Bin Values

Histograms may be filled with values of the following types:

- Primitive floating point and integer number types.
- All types that implement [Fill]
- All types that implement [FillWith]
- All types that implement [FillWithWeighted]
- All types that implement [AddAssign](std::ops::AddAssign) (as they are also [FillWith]).
- All types that implement [AddAssign](std::ops::AddAssign) and [One](num_traits::One) (as they are also [Fill]).

This crate defines the following bin value types:

- [Sum](value::Sum) : a simple bin count that counts the number of times it has been filled.
- [WeightedSum](value::WeightedSum) : as Sum but with weighted fills.
- [Mean](value::Mean) : computes the mean of the values it is filled with.
- [WeightedMean](value::WeightedMean) : as Mean but with weighted fills.

User defined bin value types are possible by implementing the [Fill], [FillWith] or [FillWithWeighted] traits.

## How to Guide

### Customize the Bin Value Type

```rust
use ndhistogram::{Histogram, ndhistogram, axis::Uniform, value::Mean};
// Create a histogram whose bin values are i32
let mut hist = ndhistogram!(Uniform::new(10, -5.0, 5.0)?; i32);
hist.fill_with(&1.0, 2);
let value: Option<&i32> = hist.value(&1.0);
assert_eq!(value, Some(&2));

// More complex value types beyond primitives are available
// "Mean" calculates the average of values it is filled with
let mut hist = ndhistogram!(Uniform::new(10, -5.0, 5.0)?; Mean);
hist.fill_with(&1.0, 1.0);
hist.fill_with(&1.0, 3.0);
assert_eq!(hist.value(&1.0).unwrap().mean(), 2.0);

// for other examples see the documentation of Sum, WeightedSum and WeightedMean

// user defined value types are possible by implementing
// Fill, FillWith or FillWithWeighted traits
```

### Create and Use a 2D Histogram

```rust
use ndhistogram::{Histogram, ndhistogram, axis::Uniform};
// create a 2D histogram
let mut hist = ndhistogram!(Uniform::new(10, -5.0, 5.0)?, Uniform::new(10, -5.0, 5.0)?);
// fill 2D histogram
hist.fill(&(1.0, 2.0));
// read back the histogram values
let x1_y2 = hist.value(&(1.0, 2.0));
// higher dimensions are possible with additional arguments to ndhistogram
```

### Create a Histogram with a Discrete Axis
```rust
use ndhistogram::{Histogram, ndhistogram, axis::Category};
let mut hist = ndhistogram!(Category::new(vec![0, 2, 4]));
hist.fill_with(&2, 42.0);
hist.fill_with(&1, 128.0);
assert_eq!(hist.value(&2), Some(&42.0));
assert_eq!(hist.value(&1), Some(&128.0));
assert_eq!(hist.value(&3), Some(&128.0));
// 1 and 3 give the same answer as they are both mapped to the overflow bin
// For a version with no overflow bins use CategoryNoFlow

// The Category type can be any hashable type, for example string
let mut hist = ndhistogram!(Category::new(vec!["Red", "Blue", "Green"]));
hist.fill(&"Red");
assert_eq!(hist.value(&"Red"), Some(&1.0));
```

### Create a Histogram with Variable Sized Bins

```rust
use ndhistogram::{Histogram, ndhistogram, axis::Variable};
let mut hist = ndhistogram!(Variable::new(vec![0.0, 1.0, 3.0, 6.0])?);
for x in 0..6 {
    hist.fill(&f64::from(x));
}
assert_eq!(hist.value(&0.0), Some(&1.0));
assert_eq!(hist.value(&1.0), Some(&2.0));
assert_eq!(hist.value(&3.0), Some(&3.0));
```

### Create a Histogram with a Periodic or Cyclic Axis

```rust
use std::f64::consts::PI;
use ndhistogram::{Histogram, ndhistogram, axis::UniformCyclic};
let mut hist = ndhistogram!(UniformCyclic::<f64>::new(10, 0.0, 2.0*PI)?);
hist.fill(&PI);
hist.fill(&-PI);
// +pi and -pi are mapped onto the same value
assert_eq!(hist.value(&-PI), Some(&2.0));
assert_eq!(hist.value(&PI), Some(&2.0));
```

### Create a Sparse Histogram

```rust
use ndhistogram::{Histogram, sparsehistogram, axis::Uniform};
// This histogram has 1e18 bins, too many to allocate with a normal histogram
let mut histogram_with_lots_of_bins = sparsehistogram!(
    Uniform::new(1_000_000, -5.0, 5.0)?,
    Uniform::new(1_000_000, -5.0, 5.0)?,
    Uniform::new(1_000_000, -5.0, 5.0)?
);
histogram_with_lots_of_bins.fill(&(1.0, 2.0, 3.0));
// read back the filled value
assert_eq!(histogram_with_lots_of_bins.value(&(1.0, 2.0, 3.0)).unwrap(), &1.0);
// unfilled bins will return None
assert!(histogram_with_lots_of_bins.value(&(0.0, 0.0, 0.0)).is_none());
```

### Merge Histograms

```rust
use ndhistogram::{Histogram, ndhistogram, axis::Uniform};
let mut hist1 = ndhistogram!(Uniform::<f64>::new(10, -5.0, 5.0)?);
let mut hist2 = ndhistogram!(Uniform::<f64>::new(10, -5.0, 5.0)?);
hist1.fill_with(&0.0, 2.0);
hist2.fill_with(&0.0, 3.0);
let combined_hist = (hist1 + &hist2).expect("Axes are compatible");
```

### Iterate over Histogram Bins in Parallel

```rust
#[cfg(feature = "rayon")] {
use rayon::prelude::*;
use ndhistogram::{Histogram, ndhistogram, axis::Uniform};
let mut histogram = ndhistogram!(Uniform::<f64>::new(10, -5.0, 5.0)?);
let sum: f64 = histogram.par_iter().map(|bin| bin.value).sum();
// see also: par_iter_mut, par_values, par_values_mut.
assert_eq!(sum, 0.0);
```
Requires "rayon" feature enabled.

## Crate Feature Flags
All cargo features of this crate are off by default.
The following features can be enabled in your `Cargo.toml`:
  - [serde] : enable support for histogram serialization and deserialization.
  - [rayon] : enable parallel iteration over histograms.

## How to contribute

If you discover a bug in this crate or a mistake in the documentation please either
[open an issue](https://github.com/davehadley/ndhistogram/issues) or
[submit a pull request](https://github.com/davehadley/ndhistogram/pulls).

If you want to request or add a new feature please
[open an issue](https://github.com/davehadley/ndhistogram/issues).


<!-- cargo-sync-readme end -->

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
