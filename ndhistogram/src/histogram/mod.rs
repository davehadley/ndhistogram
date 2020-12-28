use crate::axes::Axes;

pub struct Item<'a, T, V> {
    pub index: usize,
    pub bin: Option<T>,
    pub value: &'a V,
}
pub trait Histogram<'a, A: Axes + 'a, V: 'a> {
    type Values: Iterator<Item = &'a V>;

    fn axes(&self) -> &A;

    fn value_at_index(&self, index: usize) -> Option<&V>;

    fn value(&self, coordinate: A::Coordinate) -> Option<&V> {
        let index = self.axes().index(coordinate);
        self.value_at_index(index)
    }

    fn values(&'a self) -> Self::Values;

    fn iter(&'a self) -> Box<dyn Iterator<Item = Item<'a, A::BinRange, V>> + 'a> {
        Box::new(self.axes().iter().map(move |(index, binrange)| Item {
            index,
            bin: binrange,
            value: self.value_at_index(index).unwrap(),
        }))
    }
}

pub trait Fill<A: Axes> {
    fn fill(&mut self, coordinate: A::Coordinate);
}

pub trait FillWeight<A: Axes, W> {
    fn fill_weight(&mut self, coordinate: A::Coordinate, weight: W);
}

mod arrayhistogram;
pub use arrayhistogram::ArrayHistogram;
