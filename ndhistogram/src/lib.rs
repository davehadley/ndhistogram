//! Multi-dimensional histogramming for Rust.
//!
//! Current features include:
//! - Histograms with any number of dimensions from 1 up to 21 dimensions.
//! - Continuous (eg represented by a floating point number) or discrete axes (eg a category represented by a string value or enum).
//! - Flexible bin values including any primitive number type, or a user-defined struct.
//! - Unweighted and weighted filling of histograms.
//!
//! ## Quick-start
//!
//! ```rust
//! use ndhistogram::{Histogram, axis::Axis, ndhistogram, axis::Uniform, axis::Category};
//!
//! // create a 1D histogram with 10 equally sized bins between -5 and 5
//! let mut hist = ndhistogram!(Uniform::new(10, -5.0, 5.0));
//! // fill this histogram with a single value
//! hist.fill(&1.0);
//! // fill this histogram with weights
//! hist.fill_weight(&2.0, 4.0);
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
//!
//! // Several axis types are available
//! let mut hist = ndhistogram!(Category::new(vec!["Red", "Blue", "Green"]));
//! hist.fill(&"Red");
//! let red_value = hist.value(&"Red");
//! // and user axis types may be created by implementing the Axis trait
//!
//! // The Histogram bin value type is configurable
//! let mut hist = ndhistogram!(Uniform::new(10, -5.0, 5.0); i32);
//! hist.fill_weight(&1.0, 2);
//! let value: Option<&i32> = hist.value(&1.0);
//! // and user defined value types are possible by implementing Fill and FillWeight traits
//!
//! ```
//!
//! ## Overview
//!
//! A histogram is composed of two objects:
//! - An [Axis](crate::axis::Axis) for 1D histograms or set of [Axes] for higher dimensional histograms.
//! The [Axes] and [Axis](crate::axis::Axis) map from coodinate space (eg \[x,y,z\]) to an integer bin number.
//! - The histogram bin values. These are any type that implements [Fill] or [FillWeight] (including integer and floating number types).

#![doc(issue_tracker_base_url = "https://github.com/davehadley/rust-hist/issues")]
#![cfg_attr(
    debug_assertions,
    warn(
        missing_debug_implementations,
        rust_2018_idioms,
        unreachable_pub,
        unused_import_braces
    ),
    deny(unsafe_code, macro_use_extern_crate),
    warn(
        missing_docs,
        missing_crate_level_docs,
        missing_doc_code_examples,
        missing_docs
    )
)]

mod axes;
pub mod axis;
mod histogram;

pub use axes::Axes;
pub use histogram::fill::Fill;
pub use histogram::fill::FillWeight;
pub use histogram::histogram::Histogram;
pub use histogram::histogram::Item;
pub use histogram::vechistogram::VecHistogram;

#[macro_use]
mod macros;
