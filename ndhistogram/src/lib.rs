#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports),
    warn(missing_debug_implementations, rust_2018_idioms)
)]

// TODO: add warning for missing_docs before publishing

mod axes;
pub mod axis;
pub mod histogram;

#[macro_use]
mod macros;

#[cfg(test)]
mod unittests;
