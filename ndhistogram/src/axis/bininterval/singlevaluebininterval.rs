use std::fmt::{Display, Formatter};

/// A bin interval that contains only a single value.
///
/// Similar to [BinInterval], except each interval only covers a single value.
/// The only exception is the Overflow bin which may be used to mean "all other bin values".
#[derive(Debug, Clone, PartialEq)]
pub enum SingleValueBinInterval<T> {
    /// An interval to represent "other values".
    Overflow,
    /// An interval including only one value.
    Bin {
        /// The value included in this interval.
        value: T,
    },
}

impl<T> SingleValueBinInterval<T> {
    /// A factory method to create a new single valued bin interval.
    pub fn new(value: T) -> SingleValueBinInterval<T> {
        SingleValueBinInterval::Bin { value }
    }

    /// A factory method to create a new overflow bin interval.
    pub fn overflow() -> SingleValueBinInterval<T> {
        SingleValueBinInterval::Overflow {}
    }

    /// Returns the value included in the interval where it is well-defined.
    ///
    /// For the overflow bin (which may cover many values), it returns None.
    pub fn value(&self) -> Option<&T> {
        match self {
            SingleValueBinInterval::Overflow => None,
            SingleValueBinInterval::Bin { value } => Some(value),
        }
    }
}

impl<T: Display> Display for SingleValueBinInterval<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            SingleValueBinInterval::Overflow => write!(f, "{{overflow}}"),
            SingleValueBinInterval::Bin { value } => write!(f, "{{{}}}", value),
        }
    }
}
