use std::{cmp::Ordering, fmt::Display};

use crate::error::AxisError;

use super::{Axis, BinInterval};

/// An axis with variable sized bins.
///
/// An axis with variable sized bins.
/// Constructed with a list of bin edges.
/// Beyond the lowest (highest) bin edges is an underflow (overflow) bin.
/// Hence this axis has num edges + 1 bins.
///
/// For floating point types, positive and negative infinities map to overflow
/// and underflow bins respectively. NaN map to the overflow bin.
///
/// # Example
/// Create a 1D histogram with 3 variable width bins between 0.0 and 7.0, plus overflow and underflow bins.
/// ```rust
///    use ndhistogram::{ndhistogram, Histogram};
///    use ndhistogram::axis::{Axis, Variable, BinInterval};
///    # fn main() -> Result<(), ndhistogram::Error> {
///    let mut hist = ndhistogram!(Variable::new(vec![0.0, 1.0, 3.0, 7.0])?; i32);
///    hist.fill(&0.0);
///    hist.fill(&1.0);
///    hist.fill(&2.0);
///    assert_eq!(
///        hist.values().copied().collect::<Vec<_>>(),
///        vec![0, 1, 2, 0, 0],
///    );
///    # Ok(()) }
/// ```
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Variable<T = f64> {
    bin_edges: Vec<T>,
}

impl<T> Variable<T>
where
    T: PartialOrd + Copy,
{
    /// Factory method to create an axis with [Variable] binning given a set of bin edges.
    ///
    /// # Panics
    ///
    /// Panics if less than 2 edges are provided or if it fails to sort the
    /// bin edges (for example if a NAN value is given).
    pub fn new<I: IntoIterator<Item = T>>(bin_edges: I) -> Result<Self, AxisError> {
        let mut bin_edges: Vec<T> = bin_edges.into_iter().collect();
        if bin_edges.len() < 2 {
            return Err(AxisError::InvalidNumberOfBinEdges);
        }
        let mut sort_failed = false;
        bin_edges.sort_by(|a, b| {
            a.partial_cmp(b).unwrap_or_else(|| {
                sort_failed = true;
                Ordering::Less
            })
        });
        if sort_failed {
            return Err(AxisError::FailedToSortBinEdges);
        }
        Ok(Self { bin_edges })
    }

    /// Low edge of axis (excluding underflow bin).
    pub fn low(&self) -> &T {
        self.bin_edges
            .first()
            .expect("Variable bin_edges unexpectedly empty")
    }

    /// High edge of axis (excluding overflow bin).
    pub fn high(&self) -> &T {
        self.bin_edges
            .last()
            .expect("Variable bin_edges unexpectedly empty")
    }
}

impl<T> Axis for Variable<T>
where
    T: PartialOrd + Copy,
{
    type Coordinate = T;
    type BinInterval = BinInterval<T>;

    #[inline]
    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
        let search_result = self.bin_edges.binary_search_by(|probe| {
            probe
                .partial_cmp(coordinate)
                .unwrap_or(std::cmp::Ordering::Less)
        });
        match search_result {
            Ok(index) => Some(index + 1),
            Err(index) => Some(index),
        }
    }

    fn num_bins(&self) -> usize {
        self.bin_edges.len() + 1
    }

    fn bin(&self, index: usize) -> Option<Self::BinInterval> {
        if index == 0 {
            Some(Self::BinInterval::underflow(*self.low()))
        } else if index == self.bin_edges.len() {
            Some(Self::BinInterval::overflow(*self.high()))
        } else if index < self.bin_edges.len() {
            Some(Self::BinInterval::new(
                self.bin_edges[index - 1],
                self.bin_edges[index],
            ))
        } else {
            None
        }
    }
}

impl<T: Display + PartialOrd + Copy> Display for Variable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Axis{{# bins={}, range=[{}, {}), class={}}}",
            self.bin_edges.len() - 1,
            self.low(),
            self.high(),
            stringify!(Variable)
        )
    }
}

impl<'a, T> IntoIterator for &'a Variable<T>
where
    Variable<T>: Axis,
{
    type Item = (usize, <Variable<T> as Axis>::BinInterval);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
