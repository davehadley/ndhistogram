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
            axes: axes,
            values: vec![V::default(); size],
        }
    }
}

impl<A: Axes, V> Histogram<A, V> for ArrayHistogram<A, V> {
    fn fill(&mut self, coordinate: &A::Coordinate) {
        todo!()
    }

    fn value(&self, coordinate: &A::Coordinate) -> &V {
        todo!()
    }
}
