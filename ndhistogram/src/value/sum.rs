use num_traits::Float;

use crate::Fill;

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Sum<T = f64> {
    sum: T,
}

impl<T: Copy> Sum<T> {
    pub fn new(sum: T) -> Self {
        Self { sum }
    }

    pub fn get(&self) -> T {
        self.sum
    }

    pub fn variance(&self) -> T {
        self.sum
    }

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
