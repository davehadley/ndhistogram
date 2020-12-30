use crate::axes::Axes;
#[derive(Debug)]
pub struct Item<T, V> {
    pub index: usize,
    pub bin: Option<T>,
    pub value: V,
}

pub trait Histogram<'a, A: Axes, V: 'a> {
    type Values: Iterator<Item = &'a V>;
    type Iter: Iterator<Item = Item<A::BinRange, &'a V>>;

    fn axes(&self) -> &A;

    fn value_at_index(&self, index: usize) -> Option<&V>;

    fn value(&self, coordinate: A::Coordinate) -> Option<&V> {
        let index = self.axes().index(coordinate);
        self.value_at_index(index)
    }

    fn values(&'a self) -> Self::Values;
    fn iter(&'a self) -> Self::Iter;
}

pub trait Fill<A: Axes> {
    fn fill(&mut self, coordinate: A::Coordinate);
}

pub trait FillWeight<A: Axes, W> {
    fn fill_weight(&mut self, coordinate: A::Coordinate, weight: W);
}

pub trait MutableHistogram<'a, A: Axes, V: 'a>: Histogram<'a, A, V> {
    type ValuesMut: Iterator<Item = &'a mut V>;
    type IterMut: Iterator<Item = Item<A::BinRange, &'a mut V>>;

    fn value_at_index_mut(&mut self, index: usize) -> Option<&mut V>;
    fn values_mut(&'a mut self) -> Self::ValuesMut;
    fn iter_mut(&'a mut self) -> Self::IterMut;
}

mod arrayhistogram;
pub use arrayhistogram::ArrayHistogram;
