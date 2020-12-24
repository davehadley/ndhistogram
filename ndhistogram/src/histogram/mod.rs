use crate::axes::Axes;
pub trait Histogram<'a, A: Axes, V: 'a> {
    type ValueIterator: Iterator<Item = &'a V>;

    fn axes(&self) -> &A;

    fn fill(&mut self, coordinate: &A::Coordinate);

    fn value_at_index(&self, index: usize) -> Option<&V>;
    fn value(&self, coordinate: &A::Coordinate) -> Option<&V> {
        let index = self.axes().index(&coordinate);
        self.value_at_index(index)
    }

    fn iter_values(&'a self) -> Self::ValueIterator;
    //fn iter_items(&'a self) { self.axes().iter_indices().zip(self.iter_values()) }
}

mod arrayhistogram;
pub use arrayhistogram::ArrayHistogram;
