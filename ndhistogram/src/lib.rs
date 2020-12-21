#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#![feature(trace_macros)]

mod axes;
pub mod axis;
pub mod histogram;

#[macro_use]
mod macros;

#[cfg(test)]
mod unittests;
