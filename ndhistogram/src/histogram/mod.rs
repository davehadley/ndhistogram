use crate::axes::Axes;
#[derive(Debug)]
pub struct Item<'a, T, V> {
    pub index: usize,
    pub bin: Option<T>,
    pub value: &'a V,
}

#[derive(Debug)]
pub struct ItemMut<'a, T, V> {
    pub index: usize,
    pub bin: Option<T>,
    pub value: &'a mut V,
}

pub trait Histogram<'a, A: Axes, V: 'a> {
    type Values: Iterator<Item = &'a V>;
    type Iter: Iterator<Item = Item<'a, A::BinRange, V>>;

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
    type IterMut: Iterator<Item = ItemMut<'a, A::BinRange, V>>;

    fn value_at_index_mut(&mut self, index: usize) -> Option<&mut V>;
    fn values_mut(&'a mut self) -> Self::ValuesMut;
    fn iter_mut(&'a mut self) -> Self::IterMut;

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
