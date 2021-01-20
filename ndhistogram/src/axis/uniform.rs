use std::fmt::{Debug, Display};

use num_traits::Float;

use super::{Axis, BinInterval};

use serde::{Deserialize, Serialize};

/// An axis with equal sized bins.
///
/// An axis with N equally spaced, equal sized, bins between (low, high].
/// Below (above) this range is an underflow (overflow) bin.
/// Hence this axis has N+2 bins.
///
/// # Example
/// Create a 1D histogram with 10 uniform bins between -5.0 and 5.0, plus overflow and underflow bins.
/// ```rust
///    use ndhistogram::{ndhistogram, Histogram};
///    use ndhistogram::axis::{Axis, Uniform, BinInterval};
///    let hist = ndhistogram!(Uniform::new(10, -5.0, 5.0));
///    let axis = &hist.axes().as_tuple().0;
///    assert_eq!(axis.bin(0), Some(BinInterval::underflow(-5.0)));
///    assert_eq!(axis.bin(1), Some(BinInterval::new(-5.0, -4.0)));
///    assert_eq!(axis.bin(11), Some(BinInterval::overflow(5.0)));
///
/// ```
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct Uniform<T = f64> {
    num: usize,
    low: T,
    high: T,
}

impl<T> Uniform<T>
where
    T: PartialOrd,
{
    /// Factory method to create an axis with num uniformly spaced bins in the range [low, high). Under/overflow bins cover values outside this range.
    ///
    /// # Panics
    /// Panics if num bins == 0 or low == high.
    pub fn new(num: usize, low: T, high: T) -> Self {
        if num == 0 {
            panic!("Invalid axis num bins ({})", num);
        }
        if low == high {
            panic!("Invalid axis range (low == high)");
        }
        if low > high {
            Self { num, high, low }
        } else {
            Self { num, low, high }
        }
    }
}

impl<T> Uniform<T> {
    /// Low edge of axis (excluding underflow bin).
    pub fn low(&self) -> &T {
        &self.low
    }

    /// High edge of axis (excluding overflow bin).
    pub fn high(&self) -> &T {
        &self.high
    }
}

// TODO: relax float restriction or add implementation for Integers
impl<T: Float> Axis for Uniform<T> {
    type Coordinate = T;
    type BinInterval = BinInterval<T>;

    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
        let frac = (*coordinate - self.low) / (self.high - self.low);
        if frac < T::zero() {
            return Some(0);
        } else if frac >= T::one() {
            return Some(self.num + 1);
        }
        let idx: T =
            T::from(self.num).expect("num bins conversion to bin value type always succeed") * frac;
        Some(
            (idx.to_usize()
                .expect("bin number can always be converted to a valid usize"))
                + 1,
        )
    }

    fn num_bins(&self) -> usize {
        self.num + 2
    }

    fn bin(&self, index: usize) -> std::option::Option<<Self as Axis>::BinInterval> {
        if index == 0 {
            return Some(Self::BinInterval::underflow(self.low));
        } else if index == (self.num + 1) {
            return Some(Self::BinInterval::overflow(self.high));
        } else if index > (self.num + 1) {
            return None;
        }
        let start =
            self.low + (T::from(index - 1)?) * (self.high - self.low) / (T::from(self.num)?);
        let end = self.low + (T::from(index)?) * (self.high - self.low) / (T::from(self.num)?);
        Some(Self::BinInterval::new(start, end))
    }

    fn indices(&self) -> Box<dyn Iterator<Item = usize>> {
        Box::new(0..self.num_bins())
    }
}

impl<T: Display> Display for Uniform<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Axis{{# bins={}, range=[{}, {}), class={}}}",
            self.num,
            self.low,
            self.high,
            stringify!(Uniform)
        )
    }
}

impl<'a, T: Float> IntoIterator for &'a Uniform<T> {
    type Item = (usize, <Uniform<T> as Axis>::BinInterval);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
