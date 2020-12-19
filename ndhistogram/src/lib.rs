#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod axes;
mod axis;
mod binvalues;

#[macro_use]
mod histogram;

pub use axis::Axis;
pub use axis::Uniform;
pub use binvalues::VecBinValues;
pub use histogram::Histogram;
