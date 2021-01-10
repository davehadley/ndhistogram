use num::Float;

use super::{bininterval::BinInterval, Axis, Uniform};
use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq, Debug)]
pub struct UniformNoFlow<T> {
    axis: Uniform<T>,
}

impl<T: Float> UniformNoFlow<T> {
    pub fn new(num: usize, low: T, high: T) -> Self {
        UniformNoFlow {
            axis: Uniform::new(num, low, high),
        }
    }
}

impl<T: Float> Axis for UniformNoFlow<T> {
    type Coordinate = T;
    type BinRange = BinInterval<T>;

    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
        let index = self.axis.index(coordinate)?;
        if index == 0 || index + 1 == self.axis.numbins() {
            return None;
        }
        Some(index - 1)
    }

    fn numbins(&self) -> usize {
        self.axis.numbins() - 2
    }

    fn bin(&self, index: usize) -> Option<Self::BinRange> {
        let bin = self.axis.bin(index + 1)?;
        match bin {
            BinInterval::Underflow { end: _ } => None,
            BinInterval::Overflow { start: _ } => None,
            BinInterval::Bin { start: _, end: _ } => Some(bin),
        }
    }
}

impl<'a, T: Float> IntoIterator for &'a UniformNoFlow<T> {
    type Item = (usize, <Uniform<T> as Axis>::BinRange);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Float + Display> Display for UniformNoFlow<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Axis{{# bins={}, range=[{}, {}), class={}}}",
            self.numbins(),
            self.axis.low(),
            self.axis.high(),
            stringify!(UniformNoFlow)
        )
    }
}
