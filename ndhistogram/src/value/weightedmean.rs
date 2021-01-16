use std::{
    marker::PhantomData,
    ops::{AddAssign, Div, Mul},
};

use num_traits::{abs, Float, NumOps, One, Signed};

use crate::FillWith;

use super::Weighted;

/// ndhistogram bin value computes the mean of the data samples provided when
/// filling.
///
/// Mean had 3 type parameters:
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
/// let mean = hist.value(&0.0).unwrap();
/// assert_eq!(mean.get(), 2.0); // should be the mean of [1,2,3]
/// ```
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct WeightedMean<T = f64, W = f64, O = f64, C = u32> {
    sumwt: T,
    sumwt2: T,
    sumw: W,
    sumw2: W,
    count: C,
    phantom_output_type: PhantomData<O>,
}

impl<T, W, O, C> WeightedMean<T, W, O, C>
where
    T: Copy,
    W: Copy,
    O: From<T> + From<W> + From<C> + NumOps + Signed + Copy,
    C: Copy,
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
        O::from(self.sumwt) / O::from(self.sumw)
    }

    /// Get the number of times the mean value has been filled.
    pub fn num_samples(&self) -> C {
        self.count
    }

    /// Compute the variance of the samples.
    pub fn variance_of_samples(&self) -> O {
        // weighted variance is:
        // var = ((sumwt2 - sumwt*mu) / sumw) + mu2
        let mu = self.mean();
        let mu2 = mu * mu;
        mu2 + (O::from(self.sumwt2) - (O::one() + O::one()) * O::from(self.sumwt) * mu)
            / O::from(self.sumw)
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
        self.variance_of_samples() / O::from(self.sumw)
    }

    /// Compute the standard error of the mean.
    pub fn standard_error_of_mean(&self) -> O
    where
        O: Float,
    {
        self.variance_of_mean().sqrt()
    }
}

impl<T, W, O, C> FillWith<Weighted<T, W>> for WeightedMean<T, W, O, C>
where
    T: Copy + AddAssign + Mul<W, Output = T> + Mul<T, Output = T>,
    W: Copy + AddAssign + Mul<W, Output = W>,
    C: AddAssign + One,
{
    fn fill_with(&mut self, data: Weighted<T, W>) {
        self.sumwt += data.value * data.weight;
        self.sumwt2 += data.value * data.value * data.weight;
        self.sumw += data.weight;
        self.sumw2 += data.weight * data.weight;
        self.count += C::one();
    }
}
