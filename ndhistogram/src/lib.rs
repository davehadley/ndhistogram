#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports),
    warn(missing_debug_implementations, rust_2018_idioms)
)]

// TODO: add warning for missing_docs before publishing

mod axes;
pub use axes::Axes;
pub use axes::AxesTuple;
pub mod axis;
mod histogram;

pub use histogram::arrayhistogram::ArrayHistogram;
pub use histogram::fill::Fill;
pub use histogram::fill::FillWeight;
pub use histogram::histogram::Histogram;
pub use histogram::histogram::Item;

#[macro_use]
mod macros;

#[cfg(test)]
mod unittests;
