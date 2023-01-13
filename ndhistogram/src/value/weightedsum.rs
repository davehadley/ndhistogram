use std::ops::Mul;

use num_traits::Float;

use crate::{Fill, FillWith};

/// ndhistogram bin value type that calculates a weight sum.
/// It also provides methods to keep track of the sum of weights squared.
/// This is used to provide estimates of the statistical error on the weighted
/// sum. This performs a similar function to `Sumw2` that
/// [ROOT](https://root.cern.ch/doc/master/classTH1.html) users may be familiar
/// with.
#[derive(Copy, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WeightedSum<T = f64> {
    sumw: T,
    sumw2: T,
}

impl<T: Copy> WeightedSum<T> {
    /// Factory method to create an unfilled (or zero-valued) WeightedSum.
    pub fn new() -> Self
    where
        Self: Default,
    {
        Self::default()
    }

    /// Get the current value of the weighted sum.
    pub fn get(&self) -> T {
        self.sum()
    }

    /// Get the current value of the weighted sum.
    pub fn sum(&self) -> T {
        self.sumw
    }

    /// Estimate of the variance of the weighted sum value is the sum of the
    /// weights squared.
    pub fn variance(&self) -> T {
        self.sumw2
    }

    /// Square root of the variance.
    pub fn standard_deviation<O: Float>(&self) -> O
    where
        T: Into<O>,
        O: Float,
    {
        self.variance().into().sqrt()
    }
}

impl<T: Copy + Fill> Fill for WeightedSum<T> {
    #[inline]
    fn fill(&mut self) {
        self.sumw.fill();
        self.sumw2.fill();
    }
}

impl<T, W> FillWith<W> for WeightedSum<T>
where
    T: FillWith<W> + Copy,
    W: Mul<Output = W> + Copy,
{
    #[inline]
    fn fill_with(&mut self, weight: W) {
        self.sumw.fill_with(weight);
        self.sumw2.fill_with(weight * weight);
    }
}
