use super::{Axis, BinInterval, Uniform};
use std::fmt::Debug; // TODO Display

use num_traits::{Float, Num, NumCast, NumOps};
use serde::{Deserialize, Serialize};

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
/// use ndhistogram::axis::{Axis, BinInterval, Cyclic};
/// let mut hist = ndhistogram!(Cyclic::new(4, 0.0, 360.0));
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
/// use ndhistogram::axis::{Axis, BinInterval, Cyclic};
/// let bins_per_day = 24;
/// let hours_per_bin = 1;
/// let start_at_zero = 0;
/// let four_pm = 16;
/// let mut hist = ndhistogram!(Cyclic::with_step_size(
///     bins_per_day, start_at_zero, hours_per_bin
/// ));
/// hist.fill(&40);                               // The 40th hour of the week ...
/// assert_eq!(hist.value(&four_pm), Some(&1.0)); // ... is at 4 pm.
/// ````
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct Cyclic<T = f64> {
    axis: Uniform<T>,
}

impl<T> Cyclic<T>
where
    T: PartialOrd + Num + NumCast + NumOps + Copy,
{
    /// Create a wrap-around axis with `nbins` uniformly-spaced bins in the range `[low, high)`.
    ///
    /// Only implemented for [Float]. Use [Cyclic::with_step_size] for integers.
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

impl<T> Cyclic<T> {
    /// Low edge of axis (excluding wrap-around)
    pub fn low(&self) -> &T {
        self.axis.low()
    }
    /// High edge of axis (excluding wrap-around)
    pub fn high(&self) -> &T {
        self.axis.high()
    }
}

// TODO integers?
impl<T: PartialOrd + Num + NumCast + NumOps + Copy> Axis for Cyclic<T> {
    type Coordinate = T;
    type BinInterval = BinInterval<T>;

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

    fn num_bins(&self) -> usize {
        self.axis.num_bins() - 2
    }

    fn bin(&self, index: usize) -> Option<<Self as Axis>::BinInterval> {
        self.axis.bin(index + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest(/**/bin_no,      expected_interval,
             case(0    , Some(BinInterval::new(0.00, 0.25))),
             case(1    , Some(BinInterval::new(0.25, 0.50))),
             case(2    , Some(BinInterval::new(0.50, 0.75))),
             case(3    , Some(BinInterval::new(0.75, 1.00))),
    )]
    fn bin(bin_no: usize, expected_interval: Option<BinInterval<f32>>) {
        let axis = Cyclic::new(4, 0.0, 1.0);
        assert_eq!(axis.bin(bin_no), expected_interval);
    }

    #[rstest(coordinate, expected_index,
             case(  0.0 , Some(0)),
             case(  0.09, Some(0)),
             case(  0.1 , Some(1)),
             case(  0.19, Some(1)),
             case(  0.2 , Some(2)),
             case( 10.0 , Some(0)),
             case( 20.33, Some(3)),
             case( 50.99, Some(9)),
             case( -0.1 , Some(9)),
             case( -0.19, Some(8)),
             case( -0.2 , Some(8)),
             case( -0.9 , Some(1)),
             case( -0.95, Some(0)),
             case(-10.0 , Some(0)),
             case(-10.05, Some(9)),
             case(-10.1 , Some(8)),
    )]
    fn index(coordinate: f32, expected_index: Option<usize>) {
        let axis = Cyclic::new(10, 0.0, 1.0);
        assert_eq!(axis.index(&coordinate), expected_index);
    }

    #[test]
    fn indices() {
        let n = 7;
        let axis = Cyclic::new(n, 23.4, 97.3);
        let indices = axis.indices().collect::<Vec<_>>();
        assert_eq!(indices, (0..n).collect::<Vec<_>>());
    }
}

#[cfg(test)]
mod test_histogram {
    use super::*;
    use crate::{ndhistogram, Histogram};

    #[test]
    fn wrap_float_fill() {
        let mut hist = ndhistogram!(Cyclic::new(4, 0.0, 360.0); u8);
        hist.fill(&45.0);
        hist.fill(&(45.0 + 360.0));
        hist.fill(&(45.0 - 360.0));
        assert_eq!(hist.value(&45.0), Some(&3));
        assert_eq!(hist.value_at_index(0), Some(&3));
    }

    #[test]
    fn wrap_int_fill() {
        let bins_per_day = 24;
        let hours_per_bin = 1;
        let start_at_zero = 0;
        let mut hist = ndhistogram!(Cyclic::with_step_size(
            bins_per_day,
            start_at_zero,
            hours_per_bin
        ));
        hist.fill(&40); // The 40th hour of the week ...
        assert_eq!(hist.value(&16), Some(&1.0)); // ... is at 4 pm.
    }

    #[test]
    fn wrap_float_value() {
        let mut hist = ndhistogram!(Cyclic::new(4, 0.0, 360.0); u8);
        hist.fill(&45.0);
        assert_eq!(hist.value(&45.0), Some(&1));
        assert_eq!(hist.value(&(45.0 + 360.0)), Some(&1));
        assert_eq!(hist.value(&(45.0 - 360.0)), Some(&1));
    }
}
