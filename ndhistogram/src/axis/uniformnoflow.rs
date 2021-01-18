use num_traits::Float;

use super::{Axis, BinInterval, Uniform};
use std::fmt::{Debug, Display};

/// An axis with equal sized bins and no under/overflow bins.
///
/// An axis with N equally spaced, equal sized, bins between (low, high].
/// Similar to [Uniform] but this axis has no over/underflow bins.
/// Hence it has N bins.
///
/// # Example
/// Create a 1D histogram with 10 uniformly spaced bins between -5.0 and 5.0.
/// ```rust
///    use ndhistogram::{ndhistogram, Histogram};
///    use ndhistogram::axis::{Axis, UniformNoFlow, BinInterval};
///    let hist = ndhistogram!(UniformNoFlow::new(10, -5.0, 5.0));
///    let axis = &hist.axes().0;
///    assert_eq!(axis.bin(0), Some(BinInterval::new(-5.0, -4.0)));
///    assert_eq!(axis.bin(10), None);
///
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct UniformNoFlow<T> {
    axis: Uniform<T>,
}

impl<T: Float> UniformNoFlow<T> {
    /// Factory method to create an axis with num uniformly spaced bins in the range [low, high) with no under/overflow bins.
    pub fn new(num: usize, low: T, high: T) -> Self {
        UniformNoFlow {
            axis: Uniform::new(num, low, high),
        }
    }

    /// Return the lowest bin edge.
    pub fn low(&self) -> &T {
        self.axis.low()
    }

    /// Return the highest bin edge.
    pub fn high(&self) -> &T {
        self.axis.high()
    }
}

impl<T: Float> Axis for UniformNoFlow<T> {
    type Coordinate = T;
    type BinInterval = BinInterval<T>;

    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
        let index = self.axis.index(coordinate)?;
        if index == 0 || index + 1 == self.axis.num_bins() {
            return None;
        }
        Some(index - 1)
    }

    fn num_bins(&self) -> usize {
        self.axis.num_bins() - 2
    }

    fn bin(&self, index: usize) -> Option<Self::BinInterval> {
        let bin = self.axis.bin(index + 1)?;
        match bin {
            BinInterval::Underflow { end: _ } => None,
            BinInterval::Overflow { start: _ } => None,
            BinInterval::Bin { start: _, end: _ } => Some(bin),
        }
    }
}

impl<'a, T: Float> IntoIterator for &'a UniformNoFlow<T> {
    type Item = (usize, <Uniform<T> as Axis>::BinInterval);
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
            self.num_bins(),
            self.axis.low(),
            self.axis.high(),
            stringify!(UniformNoFlow)
        )
    }
}
