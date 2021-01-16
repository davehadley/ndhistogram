use std::ops::AddAssign;

use num_traits::One;

/// Fill a histogram bin value with unit weight.
///
/// Values that may be stored in a [Histogram](crate::Histogram) should
/// implement this trait to allow that histogram to be filled.
/// A blanket implementation is provided for types that implement [One]
/// and [AddAssign] traits. See also [FillWith].
pub trait Fill {
    /// Fill this value with unit weight.
    /// For a simple number type this means simply increment by one.
    fn fill(&mut self);
}

impl<T: One + AddAssign> Fill for T {
    fn fill(&mut self) {
        *self += Self::one();
    }
}

/// Fill a histogram bin value with some data.
///
/// Fill a histogram with some value. This trait has a blanket implementation
/// for [AddAssign].
/// In the case of primitive histogram values, this is equivalent to a weighted
/// fill.
pub trait FillWith<D> {
    /// Fill this value with some data.
    /// For a simple number type means adding the weight.
    fn fill_with(&mut self, value: D);
}

impl<D> FillWith<D> for D
where
    Self: AddAssign<D>,
{
    fn fill_with(&mut self, data: D) {
        *self += data;
    }
}

/// Fill a histogram with some weighted value.
///
/// As [FillWith], but for instances where the value may also be weighted.
/// For example, see [WeightedMean](crate::value::WeightedMean).
pub trait FillWithWeighted<D, W> {
    /// Fill a histogram with some weighted value.
    fn fill_with_weighted(&mut self, data: D, weight: W);
}
