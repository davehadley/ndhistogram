use crate::{axes::Axes, axis::Axis};

use super::fill::{Fill, FillWeight};

// TODO: Using generic associated types would give a cleaner interface and avoid boxing the iterators
// https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md
pub type Values<'a, V> = Box<dyn Iterator<Item = &'a V> + 'a>;
pub type Iter<'a, A, V> = Box<dyn Iterator<Item = Item<<A as Axis>::BinInterval, &'a V>> + 'a>;

pub type ValuesMut<'a, V> = Box<dyn Iterator<Item = &'a mut V> + 'a>;
pub type IterMut<'a, A, V> =
    Box<dyn Iterator<Item = Item<<A as Axis>::BinInterval, &'a mut V>> + 'a>;

pub trait Histogram<A: Axes, V> {
    fn axes(&self) -> &A;

    fn value_at_index(&self, index: usize) -> Option<&V>;

    fn value(&self, coordinate: &A::Coordinate) -> Option<&V> {
        let index = self.axes().index(coordinate)?;
        self.value_at_index(index)
    }

    fn values(&self) -> Values<'_, V>;
    fn iter(&self) -> Iter<'_, A, V>;

    fn value_at_index_mut(&mut self, index: usize) -> Option<&mut V>;
    fn value_mut(&mut self, coordinate: &A::Coordinate) -> Option<&mut V> {
        let index = self.axes().index(coordinate)?;
        self.value_at_index_mut(index)
    }

    fn values_mut(&mut self) -> ValuesMut<'_, V>;
    fn iter_mut(&mut self) -> IterMut<'_, A, V>;

    fn fill(&mut self, coordinate: &A::Coordinate)
    where
        V: Fill,
    {
        if let Some(value) = self.value_mut(coordinate) {
            value.fill()
        }
    }

    fn fill_weight<W>(&mut self, coordinate: &A::Coordinate, weight: W)
    where
        V: FillWeight<W>,
    {
        if let Some(value) = self.value_mut(coordinate) {
            value.fill_weight(weight)
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Item<T, V> {
    pub index: usize,
    pub bin: T,
    pub value: V,
}

impl<T, V> Item<T, V> {
    pub fn new(index: usize, bin: T, value: V) -> Item<T, V> {
        Item { index, bin, value }
    }
}
