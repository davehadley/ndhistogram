use std::{
    fmt::Display,
    ops::{AddAssign, Index},
};

pub mod arrayhistogram;
pub mod fill;

#[allow(clippy::module_inception)] // shut up clippy
pub mod histogram;
