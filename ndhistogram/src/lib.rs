#![cfg_attr(
    debug_assertions,
    warn(missing_debug_implementations, rust_2018_idioms, unreachable_pub),
    deny(unsafe_code, macro_use_extern_crate),
    //warn(missing_docs, missing_crate_level_docs, missing_doc_code_examples, missing_docs),
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
