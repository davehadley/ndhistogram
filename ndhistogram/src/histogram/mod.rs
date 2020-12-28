use crate::axes::Axes;

pub struct Item<T, V> {
    pub index: usize,
    pub bin: Option<T>,
    pub value: V,
}
pub trait Histogram<'a, A: Axes + 'a, V: 'a> {
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

pub trait MutableHistogram<'a, A: Axes + 'a, V: 'a>: Histogram<'a, A, V> {
    type ValuesMut: Iterator<Item = &'a mut V>;

    fn value_at_index_mut(&'a mut self, index: usize) -> Option<&'a mut V>;
    fn values(&'a mut self) -> Self::ValuesMut;

    // fn iter_mut(&'a mut self) -> Box<dyn Iterator<Item = Item<A::BinRange, &'a mut V>> + 'a> {
    //     Box::new(self.axes().iter().map(|(index, binrange)| Item {
    //         index,
    //         bin: binrange,
    //         value: self.value_at_index_mut(index).unwrap(),
    //     }))
    // }
}

mod arrayhistogram;
pub use arrayhistogram::ArrayHistogram;
