use std::{
    convert::TryFrom,
    fmt::Display,
    ops::{Range, RangeFrom, RangeTo},
};
#[derive(Debug, PartialEq, Clone)]
pub enum BinInterval<T> {
    Underflow { end: T },
    Overflow { start: T },
    Bin { start: T, end: T },
}

impl<T> BinInterval<T> {
    pub fn underflow(end: T) -> Self {
        Self::Underflow { end }
    }
    pub fn overflow(start: T) -> Self {
        Self::Overflow { start }
    }
    pub fn new(start: T, end: T) -> Self {
        Self::Bin { start, end }
    }
}

impl<T: Copy> BinInterval<T> {
    pub fn start(&self) -> Option<T> {
        match self {
            BinInterval::Underflow { end: _ } => None,
            BinInterval::Overflow { start } => Some(*start),
            BinInterval::Bin { start, end: _ } => Some(*start),
        }
    }

    pub fn end(&self) -> Option<T> {
        match self {
            BinInterval::Underflow { end } => Some(*end),
            BinInterval::Overflow { start: _ } => None,
            BinInterval::Bin { start: _, end } => Some(*end),
        }
    }
}

impl<T> From<Range<T>> for BinInterval<T> {
    fn from(other: Range<T>) -> Self {
        BinInterval::Bin {
            start: other.start,
            end: other.end,
        }
    }
}

impl<T> From<RangeTo<T>> for BinInterval<T> {
    fn from(other: RangeTo<T>) -> Self {
        BinInterval::Underflow { end: other.end }
    }
}

impl<T> From<RangeFrom<T>> for BinInterval<T> {
    fn from(other: RangeFrom<T>) -> Self {
        BinInterval::Overflow { start: other.start }
    }
}

impl<T> TryFrom<BinInterval<T>> for Range<T> {
    type Error = ();

    fn try_from(value: BinInterval<T>) -> Result<Self, Self::Error> {
        if let BinInterval::Bin { start, end } = value {
            return Ok(Range { start, end });
        }
        Err(())
    }
}

impl<T> TryFrom<BinInterval<T>> for RangeTo<T> {
    type Error = ();

    fn try_from(value: BinInterval<T>) -> Result<Self, Self::Error> {
        if let BinInterval::Underflow { end } = value {
            return Ok(RangeTo { end });
        }
        Err(())
    }
}

impl<T> TryFrom<BinInterval<T>> for RangeFrom<T> {
    type Error = ();

    fn try_from(value: BinInterval<T>) -> Result<Self, Self::Error> {
        if let BinInterval::Overflow { start } = value {
            return Ok(RangeFrom { start });
        }
        Err(())
    }
}

impl<T: Display> Display for BinInterval<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinInterval::Underflow { end } => write!(f, "(-inf, {})", end),
            BinInterval::Overflow { start } => write!(f, "[{}, inf)", start),
            BinInterval::Bin { start, end } => write!(f, "[{}, {})", start, end),
        }
    }
}
