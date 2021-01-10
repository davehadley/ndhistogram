#![cfg_attr(
    debug_assertions,
    warn(missing_debug_implementations, rust_2018_idioms)
)]

// TODO: add warning for missing_docs before publishing

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
