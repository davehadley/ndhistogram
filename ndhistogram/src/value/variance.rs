use num_traits::Float;

pub trait Variance<T> {
    fn variance(&self) -> T;

    fn standard_deviation(&self) -> T
    where
        T: Float,
    {
        self.variance().sqrt()
    }
}
