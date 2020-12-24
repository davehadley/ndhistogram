use std::ops::AddAssign;

use num::One;

use super::Histogram;
use crate::axes::Axes;
pub struct ArrayHistogram<A, V> {
    axes: A,
    values: Vec<V>,
}

impl<A: Axes, V: Default + Clone> ArrayHistogram<A, V> {
    pub fn new(axes: A) -> ArrayHistogram<A, V> {
        let size = axes.size();
        ArrayHistogram {
            axes,
            values: vec![V::default(); size],
        }
    }
}

impl<A: Axes, V: One + AddAssign> Histogram<A, V> for ArrayHistogram<A, V> {
    fn fill(&mut self, coordinate: &A::Coordinate) {
        let index = self.axes.index(coordinate);
        self.values[index] += V::one();
    }

    fn value(&self, coordinate: &A::Coordinate) -> Option<&V> {
        let index = self.axes.index(coordinate);
        self.values.get(index)
    }
}
