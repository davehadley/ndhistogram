use num_traits::Float;

use crate::Fill;

/// ndhistogram bin value type for filling unweighted values.
/// Analogous to [WeightedSum]. Methods returning variance and standard
/// deviation assume Poisson statistics.
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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

    /// Get the current value.
    pub fn get(&self) -> T {
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
    fn fill(&mut self) {
        self.sum.fill();
    }
}
