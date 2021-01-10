//! N-dimensional histograms
//!
//! Multi-dimensional histogramming for Rust applications.
//!
//! Current features include:
//!     - Histograms with any number of dimensions from 1 up to 21 dimensions.
//!     - Continuous (eg represented by a floating point number) or discrete axes (eg a category represented by a string value).
//!     - Flexible bin values including any primitive number type, or a user-defined struct.  
//!     - Unweighted and weighted filling of histograms.
//!

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
