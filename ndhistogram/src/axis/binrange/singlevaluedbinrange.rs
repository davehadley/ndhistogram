use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SingleValuedBinRange<T> {
    Overflow,
    Bin { value: T },
}

impl<T> SingleValuedBinRange<T> {
    pub fn new(value: T) -> SingleValuedBinRange<T> {
        SingleValuedBinRange::Bin { value }
    }

    pub fn overflow() -> SingleValuedBinRange<T> {
        SingleValuedBinRange::Overflow {}
    }

    pub fn value(&self) -> Option<&T> {
        match self {
            SingleValuedBinRange::Overflow => None,
            SingleValuedBinRange::Bin { value } => Some(value),
        }
    }
}

impl<T: Display> Display for SingleValuedBinRange<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            SingleValuedBinRange::Overflow => write!(f, "{{overflow}}"),
            SingleValuedBinRange::Bin { value } => write!(f, "{{{}}}", value),
        }
    }
}
