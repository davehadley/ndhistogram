use super::{Axis, BinInterval, Uniform};
use std::fmt::{Debug, Display};

use num_traits::{Float, Num, NumCast, NumOps};

/// A wrap-around axis with equal-sized bins.
///
/// An axis with `N` equally-spaced, equal-sized bins, in `[low, high)`.
/// Entries outside this interval get wrapped around.
/// There are no overflow bins so this axis has exactly `N` bins.
///
/// # Examples
/// 1D histogram with 4 bins distributed around a circle.
/// ```
/// use ndhistogram::{ndhistogram, Histogram};
/// use ndhistogram::axis::{Axis, BinInterval, UniformCyclic};
/// let mut hist = ndhistogram!(UniformCyclic::new(4, 0.0, 360.0));
/// hist.fill(& 45.0         ); // Add entry at 45 degrees
/// hist.fill(&(45.0 + 360.0)); // Add entry at 45 degrees + one whole turn
/// hist.fill(&(45.0 - 360.0)); // Add entry at 45 degrees + one whole turn backwards
/// // All 3 above entries end up in the same bin
/// assert_eq!(hist.value(&45.0), Some(&3.0));
/// // Lookup also wraps around
/// assert_eq!(hist.value(&(45.0 + 360.0)), Some(&3.0));
/// assert_eq!(hist.value(&(45.0 - 360.0)), Some(&3.0));
/// ```
/// Time of day
/// ```
/// use ndhistogram::{ndhistogram, Histogram};
/// use ndhistogram::axis::{Axis, BinInterval, UniformCyclic};
/// let bins_per_day = 24;
/// let hours_per_bin = 1;
/// let start_at_zero = 0;
/// let four_pm = 16;
/// let mut hist = ndhistogram!(UniformCyclic::with_step_size(
///     bins_per_day, start_at_zero, hours_per_bin
/// ));
/// hist.fill(&40);                               // The 40th hour of the week ...
/// assert_eq!(hist.value(&four_pm), Some(&1.0)); // ... is at 4 pm.
/// ````
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UniformCyclic<T = f64> {
    axis: Uniform<T>,
}

impl<T> UniformCyclic<T>
where
    T: PartialOrd + Num + NumCast + NumOps + Copy,
{
    /// Create a wrap-around axis with `nbins` uniformly-spaced bins in the range `[low, high)`.
    ///
    /// Only implemented for [Float]. Use [UniformCyclic::with_step_size] for integers.
    ///
    /// For floating point types, infinities and NaN do not map to any bin.
    ///
    /// # Panics
    /// Panics under the same conditions as [Uniform::new].
    pub fn new(nbins: usize, low: T, high: T) -> Self
    where
        T: Float,
    {
        Self {
            axis: Uniform::new(nbins, low, high),
        }
    }

    /// Create a wrap-around axis with `nbins` uniformly-spaced bins in the range `[low, low+num*step)`.
    /// # Panics
    /// Panics under the same conditions as [Uniform::new].
    pub fn with_step_size(nbins: usize, low: T, step: T) -> Self {
        Self {
            axis: Uniform::with_step_size(nbins, low, step),
        }
    }
}

impl<T> UniformCyclic<T> {
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

impl<T: PartialOrd + Num + NumCast + NumOps + Copy> Axis for UniformCyclic<T> {
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
    fn bin(&self, index: usize) -> Option<<Self as Axis>::BinInterval> {
        self.axis.bin(index + 1)
    }
}

impl<T> Display for UniformCyclic<T>
where
    T: PartialOrd + NumCast + NumOps + Copy + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Axis{{# bins={}, range=[{}, {}), class={}}}",
            self.axis.num_bins(),
            self.axis.low(),
            self.axis.high(),
            stringify!(UniformCyclic)
        )
    }
}

impl<'a, T> IntoIterator for &'a UniformCyclic<T>
where
    UniformCyclic<T>: Axis,
{
    type Item = (usize, <UniformCyclic<T> as Axis>::BinInterval);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
