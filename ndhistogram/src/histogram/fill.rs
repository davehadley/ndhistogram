use std::ops::AddAssign;

use num::One;

pub trait Fill {
    fn fill(&mut self);
}

impl<T: One + AddAssign> Fill for T {
    fn fill(&mut self) {
        *self += Self::one();
    }
}

pub trait FillWeight<W> {
    fn fill_weight(&mut self, weight: W);
}

impl<W> FillWeight<W> for W
where
    Self: AddAssign<W>,
{
    fn fill_weight(&mut self, weight: W) {
        *self += weight;
    }
}
