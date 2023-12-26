use std::{
    marker::PhantomData,
    ops::{AddAssign, Div, Mul},
};

use num_traits::{Float, NumOps, One, Signed};

use crate::FillWithWeighted;

/// ndhistogram bin value computes the mean of the data samples provided when
/// filling.
///
/// Mean has 4 type parameters:
/// - the type that is being averaged,
/// - the type of the weights that are being filled,
/// - the type of the output when calculating the mean and its uncertainty,
/// - the type that counts the number of fills.
///
/// This allows, for example, integers to be used when filling or counting,
/// but a floating point type to compute the mean.
/// In most cases, you will only need to specify the first two type parameters as
/// sensible defaults are set for the second two type parameters.
///
///
/// # Example
/// ```rust
/// use ndhistogram::{ndhistogram, Histogram, axis::Uniform, value::WeightedMean};
///
/// # fn main() -> Result<(), ndhistogram::Error> {
/// // create a histogram and fill it with some values
/// let mut hist = ndhistogram!(Uniform::new(10, 0.0, 10.0)?; WeightedMean<i32, i32>);
/// hist.fill_with_weighted(&0.0, 2, 1);
/// hist.fill_with_weighted(&0.0, 2, 2);
/// hist.fill_with_weighted(&0.0, 4, 3);
///
/// let weightedmean = hist.value(&0.0);
/// assert_eq!(weightedmean.unwrap().get(), 3.0);
/// # Ok(()) }
/// ```
#[derive(Copy, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        I: IntoIterator<Item = (T, W)>,
        Self: FillWithWeighted<T, W> + Default,
    {
        let mut r = Self::default();
        values
            .into_iter()
            .for_each(|it| r.fill_with_weighted(it.0, it.1));
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
        self.variance_of_samples() * O::from(self.sumw2) / (O::from(self.sumw) * O::from(self.sumw))
    }

    /// Compute the standard error of the mean.
    pub fn standard_error_of_mean(&self) -> O
    where
        O: Float,
    {
        self.variance_of_mean().sqrt()
    }
}

impl<T, W, O, C> FillWithWeighted<T, W> for WeightedMean<T, W, O, C>
where
    T: Copy + AddAssign + Mul<W, Output = T> + Mul<T, Output = T>,
    W: Copy + AddAssign + Mul<W, Output = W>,
    C: AddAssign + One,
{
    #[inline]
    fn fill_with_weighted(&mut self, value: T, weight: W) {
        self.sumwt += value * weight;
        self.sumwt2 += value * value * weight;
        self.sumw += weight;
        self.sumw2 += weight * weight;
        self.count += C::one();
    }
}
