use num_traits::Float;

pub trait Variance<T> {
    fn variance(&self) -> T;

    fn standard_deviation<O: Float>(&self) -> O
    where
        T: Into<O>,
        O: Float,
    {
        self.variance().into().sqrt()
    }
}
