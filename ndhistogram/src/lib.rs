//! ndhistogram implements multi-dimensional histograms for Rust.
//!
//! This library aims to provide a similar feature set to the C++ library
//! [boost-histogram](https://www.boost.org/doc/libs/1_75_0/libs/histogram)
//! but with an idomatic pure-Rust implementation.
//!
//! Features include:
//! - Histograms with any number of dimensions from 1 up to 21 dimensions.
//! - Continuous (eg represented by a floating point number) and discrete axis (eg a category represented by a string value or enum) types that are composable (eg you may mix discrete and continuous axes).
//! - Flexible bin values including any primitive number type, or a user-defined type.
//! - Unweighted and weighted filling of histograms.
//! - Flexible, user-definable axis types.
//! - Sparse histograms to reduce the memory footprint of high bin count, mostly empty, histograms.
//!
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! ndhistogram = "0.5.0"
//! ```
//!
//! See the [change log](https://github.com/davehadley/ndhistogram/blob/main/ndhistogram/CHANGELOG.md)
//! for differences between releases.
//! Please report any bugs in the [issues tracker](https://github.com/davehadley/ndhistogram/issues).
//!
//! ## Quick-start
//!
//! ```rust
//! use ndhistogram::{Histogram, axis::Axis, ndhistogram, axis::Uniform, axis::Category, value::Mean};
//!
//! // create a 1D histogram with 10 equally sized bins between -5 and 5
//! let mut hist = ndhistogram!(Uniform::new(10, -5.0, 5.0));
//! // fill this histogram with a single value
//! hist.fill(&1.0);
//! // fill this histogram with weights
//! hist.fill_with(&2.0, 4.0);
//! // read the histogram values
//! let x1 = hist.value(&1.0);
//! let also_x1 = hist.value_at_index(7);
//! assert_eq!(x1, also_x1);
//! // iterate the histogram values
//! for item in hist.iter() {
//!     println!("{}, {}, {}", item.index, item.bin, item.value)
//! }
//! // print the histogram to stdout
//! println!("{}", hist);
//!
//!
//! // create a 2D histogram
//! let mut hist = ndhistogram!(Uniform::new(10, -5.0, 5.0), Uniform::new(10, -5.0, 5.0));
//! // fill 2D histogram
//! hist.fill(&(1.0, 2.0));
//! // read back the histogram values
//! let x1_y2 = hist.value(&(1.0, 2.0));
//! // higher dimensions are possible with additional arguments to ndhistogram
//!
//! // Several axis types are available
//! let mut hist = ndhistogram!(Category::new(vec!["Red", "Blue", "Green"]));
//! hist.fill(&"Red");
//! let red_value = hist.value(&"Red");
//! // and user axis types may be created by implementing the Axis trait
//!
//! // The Histogram bin value type is configurable
//! let mut hist = ndhistogram!(Uniform::new(10, -5.0, 5.0); i32);
//! hist.fill_with(&1.0, 2);
//! let value: Option<&i32> = hist.value(&1.0);
//!
//! // and more complex value types beyond primitives are available
//! let mut hist = ndhistogram!(Uniform::new(10, -5.0, 5.0); Mean);
//! hist.fill_with(&1.0, 1.0);
//! hist.fill_with(&1.0, 3.0);
//! assert_eq!(hist.value(&1.0).unwrap().mean(), 2.0);
//!
//! // user defined value types are possible by implementing Fill, FillWith or FillWithWeighted traits
//!
//! ```
//!
//! ## Overview
//!
//! A [Histogram](Histogram) is composed of two components:
//! - The [Axes] which is a set of [Axis](axis::Axis) corresponding to each dimension of the histogram.
//! The [Axes] and [Axis](axis::Axis) define the binning of the histogram and are responsible for mapping from coordinate space (eg \[x,y,z\]) to an integer bin number.
//! - The histogram bin value storage. Valid bin value types including any integer and floating number type as well as user defined types that implement [Fill], [FillWith] or [FillWithWeighted].
//!
//! ### Histogram Implementations
//!
//! - [VecHistogram]: bin values are stored in a [Vec].
//! Created with the [ndhistogram] macro.
//! This is the recommended implementation for most use cases.
//! However, as memory is allocated even for empty bins,
//! this may not be practical for very high dimension histograms.
//! - [HashHistogram]: bin values are stored in a [HashMap](std::collections::HashMap).
//! Created with the [sparsehistogram] macro.
//! Useful for high dimension, mostly empty, histograms as empty bins
//! take up no memory.
//!
//! Alternative implementations are possible by implementing the [Histogram] trait.
//!
//! ### Axis Implementations
//!
//! - [Uniform](axis::Uniform)/[UniformNoFlow](axis::UniformNoFlow): equally sized bins in a some range with optional underflow/overflow bins.
//! - [Variable](axis::Variable)/[VariableNoFlow](axis::VariableNoFlow): variable sized bins with optional underflow/overflow bins.
//! - [Category](axis::Category)/[CategoryNoFlow](axis::CategoryNoFlow): a finite set of discrete values with optional overflow bin.
//!
//! User defined axes types are possible by implementing the [Axis](axis::Axis) trait.
//!
//! ### Histogram Bin Values
//!
//! Histograms may be filled with values of the following types:
//!
//! - Primitive floating point and integer number types.
//! - All types that implement [Fill]
//! - All types that implement [FillWith]
//! - All types that implement [FillWithWeighted]
//! - All types that implement [AddAssign](std::ops::AddAssign) (as they are also [FillWith]).
//! - All types that implement [AddAssign](std::ops::AddAssign) and [One](num_traits::One) ( as they are also [Fill]).
//!
//! This crate defines the following bin value types:
//!
//! - [Sum](value::Sum) : a simple bin count that counts the number of times it has been filled.
//! - [WeightedSum](value::WeightedSum) : as Sum but with weighted fills.
//! - [Mean](value::Mean) : computes the mean of the values it is filled with.
//! - [WeightedMean](value::WeightedMean) : as Mean but with weighted fills.
//!
//! User defined bin value types are possible by implementing the [Fill], [FillWith] or [FillWithWeighted] traits.

#![doc(issue_tracker_base_url = "https://github.com/davehadley/rust-hist/issues")]
#![doc(html_root_url = "https://docs.rs/ndhistogram/0.5.0")]
#![cfg_attr(
    debug_assertions,
    warn(
        missing_debug_implementations,
        rust_2018_idioms,
        unreachable_pub,
        unused_import_braces,
    ),
    deny(unsafe_code, macro_use_extern_crate),
    warn(
        missing_docs,
        missing_crate_level_docs,
        missing_doc_code_examples,
        broken_intra_doc_links,
    )
)]

mod axes;
pub mod axis;
mod histogram;

pub mod value;

pub use axes::Axes;
pub use axes::AxesTuple;
pub use histogram::fill::Fill;
pub use histogram::fill::FillWith;
pub use histogram::fill::FillWithWeighted;
pub use histogram::hashhistogram::HashHistogram;
pub use histogram::histogram::Histogram;
pub use histogram::histogram::Item;
pub use histogram::vechistogram::VecHistogram;

/// Type alias for 1D [Histogram]s returned by [ndhistogram].
pub type Hist1D<X, V = f64> = VecHistogram<AxesTuple<(X,)>, V>;

/// Type alias for 2D [Histogram]s returned by [ndhistogram].
pub type Hist2D<X, Y, V = f64> = VecHistogram<AxesTuple<(X, Y)>, V>;

/// Type alias for 3D [Histogram]s returned by [ndhistogram].
pub type Hist3D<X, Y, Z, V = f64> = VecHistogram<AxesTuple<(X, Y, Z)>, V>;

/// Type alias for ND [Histogram]s returned by [ndhistogram].
pub type HistND<A, V = f64> = VecHistogram<AxesTuple<A>, V>;

/// Type alias for 1D [Histogram]s returned by [sparsehistogram].
pub type SparseHist1D<X, V = f64> = HashHistogram<AxesTuple<(X,)>, V>;

/// Type alias for 2D [Histogram]s returned by [sparsehistogram].
pub type SparseHist2D<X, Y, V = f64> = HashHistogram<AxesTuple<(X, Y)>, V>;

/// Type alias for 3D [Histogram]s returned by [sparsehistogram].
pub type SparseHist3D<X, Y, Z, V = f64> = HashHistogram<AxesTuple<(X, Y, Z)>, V>;

/// Type alias for ND [Histogram]s returned by [sparsehistogram].
pub type SparseHistND<A, V = f64> = HashHistogram<AxesTuple<A>, V>;

/// Provides errors that may be returned by [Histogram]s.
pub mod error;

#[macro_use]
mod macros;
