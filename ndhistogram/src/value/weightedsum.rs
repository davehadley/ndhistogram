use std::ops::Mul;

use num_traits::Float;

use crate::{Fill, FillWeight};

use super::Variance;

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct WeightedSum<T = f64> {
    sumw: T,
    sumw2: T,
}

impl<T: Copy> WeightedSum<T> {
    pub fn new(sumw: T, sumw2: T) -> Self {
        Self { sumw, sumw2 }
    }

    pub fn get(&self) -> T {
        self.sumw
    }
}

impl<T: Copy> Variance<T> for WeightedSum<T> {
    fn variance(&self) -> T {
        self.sumw2
    }

    fn standard_deviation(&self) -> T
    where
        T: Float,
    {
        self.variance().sqrt()
    }
}

impl<T: Copy + Fill> Fill for WeightedSum<T> {
    fn fill(&mut self) {
        self.sumw.fill();
        self.sumw2.fill();
    }
}

impl<T, W> FillWeight<W> for WeightedSum<T>
where
    T: FillWeight<W> + Copy,
    W: Mul<Output = W> + Copy,
{
    fn fill_weight(&mut self, weight: W) {
        self.sumw.fill_weight(weight);
        self.sumw2.fill_weight(weight * weight);
    }
}
