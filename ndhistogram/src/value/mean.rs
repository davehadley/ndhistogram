use std::ops::AddAssign;

use num_traits::{Float, NumOps};

use crate::{Fill, FillWith};

use super::Variance;

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Mean<T = f64, C = u32> {
    sum: T,
    count: C,
}

impl<C: Copy, T: Copy + NumOps + From<C>> Mean<T, C> {
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
        self.sum / self.count.into()
    }

    pub fn num_samples(&self) -> C {
        self.count
    }

    pub fn variance_of_samples(&self) -> T {
        todo!()
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
        self.sum
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
    T: Copy + AddAssign,
{
    fn fill_with(&mut self, data: T) {
        self.sum += data;
        self.count += 1;
    }
}
