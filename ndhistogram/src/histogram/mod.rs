use crate::axes::Axes;
pub trait Histogram<'a, A: Axes, V: 'a> {
    type Values: Iterator<Item = &'a V>;
    //type ItemIterator: Iterator<Item = &'a V>;

    fn axes(&self) -> &A;

    fn value_at_index(&self, index: usize) -> Option<&V>;
    fn value(&self, coordinate: A::Coordinate) -> Option<&V> {
        let index = self.axes().index(coordinate);
        self.value_at_index(index)
    }

    fn values(&'a self) -> Self::Values;
    //fn iter(&'a self) -> Box<dyn Iterator<Item=>> { self.axes().indices().zip(self.values()) }
}

pub trait Fill<A: Axes> {
    fn fill(&mut self, coordinate: A::Coordinate);
}

pub trait FillWeight<A: Axes, W> {
    fn fill_weight(&mut self, coordinate: A::Coordinate, weight: W);
}

mod arrayhistogram;
pub use arrayhistogram::ArrayHistogram;
