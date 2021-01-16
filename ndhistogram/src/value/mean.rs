use std::{
    marker::PhantomData,
    ops::{AddAssign, Div, Mul},
};

use num_traits::{abs, Float, Num, NumOps, One, Signed};

use crate::{Fill, FillWith};

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Mean<T = f64, C = u32, O = f64> {
    sumw: T,
    sumw2: T,
    count: C,
    phantom_output_type: PhantomData<O>,
}

impl<C, T, O> Mean<T, C, O>
where
    O: From<T> + From<C> + NumOps + Signed + Copy,
    C: Copy,
    T: Copy,
{
    pub fn new<I>(values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        Self: FillWith<T> + Default,
    {
        let mut r = Self::default();
        values.into_iter().for_each(|it| r.fill_with(it));
        r
    }

    pub fn get(&self) -> O {
        self.mean()
    }

    pub fn mean(&self) -> <O as Div>::Output {
        O::from(self.sumw) / O::from(self.count)
    }

    pub fn num_samples(&self) -> C {
        self.count
    }

    pub fn variance_of_samples(&self) -> O {
        let mean = self.mean();
        let mean2 = mean * mean;
        let avgsum2 = O::from(self.sumw2) / O::from(self.count);
        abs(avgsum2 - mean2)
    }

    pub fn standard_deviation_of_samples(&self) -> O
    where
        O: Float,
    {
        self.variance_of_samples().sqrt()
    }

    pub fn variance_of_mean(&self) -> O {
        self.variance_of_samples() / O::from(self.count)
    }

    pub fn standard_error_of_mean(&self) -> O
    where
        O: Float,
    {
        self.variance_of_mean().sqrt()
    }
}

impl<T, C, O> FillWith<T> for Mean<T, C, O>
where
    T: Copy + AddAssign + Mul<Output = T>,
    C: AddAssign + One,
{
    fn fill_with(&mut self, data: T) {
        self.sumw += data;
        self.sumw2 += data * data;
        self.count += C::one();
    }
}
