use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum SingleValueBinInterval<T> {
    Overflow,
    Bin { value: T },
}

impl<T> SingleValueBinInterval<T> {
    pub fn new(value: T) -> SingleValueBinInterval<T> {
        SingleValueBinInterval::Bin { value }
    }

    pub fn overflow() -> SingleValueBinInterval<T> {
        SingleValueBinInterval::Overflow {}
    }

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
