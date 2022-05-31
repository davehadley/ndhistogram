use super::{Axis, BinInterval, Variable};
use std::fmt::{Debug, Display};

use num_traits::Num;
use serde::{Deserialize, Serialize};

/// A wrap-around axis with variable-sized bins.
///
/// A wrap-around axis with variable-sized bins, constructed from a list of bin
/// edges.
///
/// # Examples
/// 1D histogram with cyclic variable sized azimuthal angle binning.
/// ```
/// use ndhistogram::{ndhistogram, Histogram};
/// use ndhistogram::axis::{Axis, BinInterval, VariableCyclic};
/// use std::f64::consts::PI;
/// let mut hist = ndhistogram!(VariableCyclic::new(vec![0.0, PI/2.0, PI, 2.0*PI]); i32);
/// let angle = 0.1;
/// hist.fill(&angle); // fills the first bin
/// hist.fill(&(angle + 2.0*PI)); // wraps around and fills the same first bin
/// assert_eq!(hist.value(&angle), Some(&2));
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct VariableCyclic<T = f64> {
    axis: Variable<T>,
}

impl<T> VariableCyclic<T>
where
    T: PartialOrd + Copy,
{
    /// Create a wrap-around axis with [Variable] binning given a set of bin edges.
    ///
    /// # Panics
    ///
    /// Panics if fewer than 2 edges are provided, or if the edges cannot be
    /// sorted (for example when given NAN).
    pub fn new<I: IntoIterator<Item = T>>(bin_edges: I) -> Self {
        Self {
            axis: Variable::new(bin_edges),
        }
    }

    /// Low edge of axis (excluding wrap-around)
    #[inline]
    pub fn low(&self) -> &T {
        self.axis.low()
    }

    /// High edge of axis (excluding wrap-around)
    #[inline]
    pub fn high(&self) -> &T {
        self.axis.high()
    }
}

impl<T> Axis for VariableCyclic<T>
where
    T: PartialOrd + Copy + Num,
{
    type Coordinate = T;
    type BinInterval = BinInterval<T>;

    #[inline]
    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
        let (mut x, hi, lo) = (*coordinate, *self.axis.high(), *self.axis.low());
        let range = hi - lo;
        x = (x - lo) % range;
        if x < T::zero() {
            x = range + x;
        }
        x = x + lo;
        self.axis.index(&x).map(|n| n - 1)
    }

    #[inline]
    fn num_bins(&self) -> usize {
        self.axis.num_bins() - 2
    }

    #[inline]
    fn bin(&self, index: usize) -> Option<Self::BinInterval> {
        self.axis.bin(index + 1)
    }
}

impl<T> Display for VariableCyclic<T>
where
    T: PartialOrd + Copy + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Axis{{# bins={}, range=[{}, {}), class={}}}",
            self.axis.num_bins(),
            self.axis.low(),
            self.axis.high(),
            stringify!(VariableCyclic)
        )
    }
}

impl<'a, T> IntoIterator for &'a VariableCyclic<T>
where
    VariableCyclic<T>: Axis,
{
    type Item = (usize, <VariableCyclic<T> as Axis>::BinInterval);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
