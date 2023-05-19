use num_traits::Float;

use crate::Fill;
use crate::FillWith;

/// ndhistogram bin value type for filling unweighted values.
/// Analogous to [WeightedSum](crate::value::WeightedSum). Methods returning variance and standard
/// deviation assume Poisson statistics.
#[derive(Copy, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sum<T = f64> {
    sum: T,
}

impl<T: Copy> Sum<T> {
    /// Factory method to create an unfilled (or zero valued) Sum.
    pub fn new() -> Self
    where
        Self: Default,
    {
        Self::default()
    }

    /// Get the current value of the sum.
    pub fn get(&self) -> T {
        self.sum()
    }

    /// Get the current value.
    pub fn sum(&self) -> T {
        self.sum
    }

    /// Estimate of the variance of value assuming Poisson statistics.
    pub fn variance(&self) -> T {
        self.sum
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

impl<T: Copy + Fill> Fill for Sum<T> {
    #[inline]
    fn fill(&mut self) {
        self.sum.fill();
    }
}

impl<T, W> FillWith<W> for Sum<T>
where
    T: FillWith<W> + Copy,
    W: Copy,
{
    #[inline]
    fn fill_with(&mut self, weight: W) {
        self.sum.fill_with(weight);
    }
}
