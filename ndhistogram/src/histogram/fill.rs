use std::ops::AddAssign;

use num_traits::One;

/// Fill a histogram bin value with unit weight.
///
/// Values that may be stored in a [Histogram](crate::Histogram) should implement this trait to allow that histogram to be filled.
/// A blanket implementation is provided for types that implement [One] and [AddAssign] traits.
/// See also [FillWeight].
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

/// Fill a histogram bin value with a weight.
///
/// Values that may be stored in a [Histogram](crate::Histogram) should implement this trait to allow that histogram to be filled with weights.
/// A blanket implementation is provided for types that implement the [AddAssign] trait.
pub trait FillWeight<W> {
    /// Fill this value with some weight.
    /// For a simple number type means adding the weight.
    fn fill_weight(&mut self, weight: W);
}

impl<W> FillWeight<W> for W
where
    Self: AddAssign<W>,
{
    fn fill_weight(&mut self, weight: W) {
        *self += weight;
    }
}
