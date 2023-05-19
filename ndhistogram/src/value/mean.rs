use std::{
    marker::PhantomData,
    ops::{AddAssign, Div, Mul},
};

use num_traits::{abs, Float, NumOps, One, Signed};

use crate::FillWith;

/// This ndhistogram bin value computes the mean of the data samples provided when
/// filling.
///
/// Mean has 3 type parameters:
/// - the type that is being averaged,
/// - the type of the output when calculating the mean and its uncertainty,
/// - the type that counts the number of fills.
///
/// This allows, for example, integers to be used when filling or counting,
/// but a floating point type to compute the mean.
/// In most cases, you will only need to specify the first type as
/// sensible defaults are set for the second two type parameters.
///
///
/// # Example
/// ```rust
/// use ndhistogram::{ndhistogram, Histogram, axis::Uniform, value::Mean};
///
/// // create a histogram and fill it with some values
/// let mut hist = ndhistogram!(Uniform::new(10, 0.0, 10.0); Mean<i32>);
/// hist.fill_with(&0.0, 1);
/// hist.fill_with(&0.0, 2);
/// hist.fill_with(&0.0, 3);
///
/// let mean = hist.value(&0.0);
/// assert_eq!(mean.unwrap().get(), 2.0); // should be the mean of [1,2,3]
/// ```
#[derive(Copy, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mean<T = f64, O = f64, C = u32> {
    sumw: T,
    sumw2: T,
    count: C,
    phantom_output_type: PhantomData<O>,
}

impl<C, T, O> Mean<T, O, C>
where
    O: From<T> + From<C> + NumOps + Signed + Copy,
    C: Copy,
    T: Copy,
{
    /// Factory method to create a Mean from a set of values.
    ///
    /// Usually this will not be used as a [Histogram](crate::Histogram) will
    /// be responsible for creating and filling values.
    pub fn new<I>(values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        Self: FillWith<T> + Default,
    {
        let mut r = Self::default();
        values.into_iter().for_each(|it| r.fill_with(it));
        r
    }

    /// Get the current value of the mean.
    pub fn get(&self) -> O {
        self.mean()
    }

    /// Get the current value of the mean.
    pub fn mean(&self) -> <O as Div>::Output {
        O::from(self.sumw) / O::from(self.count)
    }

    /// Get the number of times the mean value has been filled.
    pub fn num_samples(&self) -> C {
        self.count
    }

    /// Compute the variance of the samples.
    pub fn variance_of_samples(&self) -> O {
        let mean = self.mean();
        let mean2 = mean * mean;
        let avgsum2 = O::from(self.sumw2) / O::from(self.count);
        abs(avgsum2 - mean2)
    }

    /// The square root of the variance of the samples.
    pub fn standard_deviation_of_samples(&self) -> O
    where
        O: Float,
    {
        self.variance_of_samples().sqrt()
    }

    /// The square of the standard error of the mean.
    pub fn variance_of_mean(&self) -> O {
        self.variance_of_samples() / O::from(self.count)
    }

    /// Compute the standard error of the mean.
    pub fn standard_error_of_mean(&self) -> O
    where
        O: Float,
    {
        self.variance_of_mean().sqrt()
    }
}

impl<T, C, O> FillWith<T> for Mean<T, O, C>
where
    T: Copy + AddAssign + Mul<Output = T>,
    C: AddAssign + One,
{
    #[inline]
    fn fill_with(&mut self, data: T) {
        self.sumw += data;
        self.sumw2 += data * data;
        self.count += C::one();
    }
}
