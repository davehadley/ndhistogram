use crate::error::AxisError;

use super::{Axis, BinInterval, Variable};

use std::fmt::{Debug, Display};

/// An axis with variable sized bins and no overflow bins.
///
/// An axis with variable sized bins constructed with a list of bin edges.
/// This axis has (num edges - 1) bins.
///
/// For floating point types, infinities and NaN do not map to any bin.
///
/// # Example
/// Create a 1D histogram with 3 variable width bins between 0.0 and 7.0.
/// ```rust
///    use ndhistogram::{ndhistogram, Histogram};
///    use ndhistogram::axis::{Axis, VariableNoFlow};
///    # fn main() -> Result<(), ndhistogram::Error> {
///    let mut hist = ndhistogram!(VariableNoFlow::new(vec![0.0, 1.0, 3.0, 7.0])?; i32);
///    hist.fill(&-1.0); // will be ignored as there is no underflow bin
///    hist.fill(&1.0);
///    hist.fill(&2.0);
///    assert_eq!(
///        hist.values().copied().collect::<Vec<_>>(),
///        vec![0, 2, 0],
///    );
///    # Ok(()) }
/// ```
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariableNoFlow<T = f64> {
    axis: Variable<T>,
}

impl<T: PartialOrd + Copy> VariableNoFlow<T> {
    /// Factory method to create an variable binning from a set of bin edges with no under/overflow bins.
    /// See the documentation for [Variable::new].
    ///
    /// # Panics
    /// Panics under the same conditions as [Variable::new].
    pub fn new<I: IntoIterator<Item = T>>(bin_edges: I) -> Result<Self, AxisError> {
        Ok(Self {
            axis: Variable::new(bin_edges)?,
        })
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

impl<T: PartialOrd + Copy> Axis for VariableNoFlow<T> {
    type Coordinate = T;
    type BinInterval = BinInterval<T>;

    #[inline]
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

impl<'a, T: PartialOrd + Copy> IntoIterator for &'a VariableNoFlow<T> {
    type Item = (usize, <VariableNoFlow<T> as Axis>::BinInterval);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: PartialOrd + Copy + Display> Display for VariableNoFlow<T>
where
    Self: Axis,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Axis{{# bins={}, range=[{}, {}), class={}}}",
            self.num_bins(),
            self.axis.low(),
            self.axis.high(),
            stringify!(VariableNoFlow)
        )
    }
}
