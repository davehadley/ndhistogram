use std::ops::{AddAssign, Mul};

use num_traits::{abs, Float, Num, NumOps, Signed};

use crate::{Fill, FillWith};

use super::Variance;

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Mean<T = f64, C = u32> {
    sumw: T,
    sumw2: T,
    count: C,
}

impl<C: Copy, T: Copy + Signed + NumOps + From<C>> Mean<T, C> {
    pub fn new<I>(values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        Self: FillWith<T> + Default,
    {
        let mut r = Self::default();
        values.into_iter().for_each(|it| r.fill_with(it));
        r
    }

    pub fn get(&self) -> T {
        self.mean()
    }

    pub fn mean(&self) -> T {
        self.sumw / self.count.into()
    }

    pub fn num_samples(&self) -> C {
        self.count
    }

    pub fn variance_of_samples(&self) -> T {
        let mean = self.mean();
        let mean2 = mean * mean;
        let avgsum2 = self.sumw2 / self.count.into();
        abs(avgsum2 - mean2)
    }

    pub fn standard_deviation_of_samples(&self) -> T
    where
        T: Float,
    {
        self.variance_of_samples().sqrt()
    }

    pub fn variance_of_mean(&self) -> T {
        self.variance_of_samples() / self.count.into()
    }

    pub fn standard_error_of_mean(&self) -> T
    where
        T: Float,
    {
        self.variance_of_mean().sqrt()
    }
}

impl<T: Copy> Variance<T> for Mean<T> {
    fn variance(&self) -> T {
        self.sumw
    }

    fn standard_deviation(&self) -> T
    where
        T: Float,
    {
        self.variance().sqrt()
    }
}

impl<T> FillWith<T> for Mean<T>
where
    T: Copy + AddAssign + Mul<Output = T>,
{
    fn fill_with(&mut self, data: T) {
        self.sumw += data;
        self.sumw2 += data * data;
        self.count += 1;
    }
}
