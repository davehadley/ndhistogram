use std::{
    fmt::Display,
    ops::{AddAssign, Index},
};

use crate::{axes::Axes, axis::Axis};

// TODO: Replace with trait alias when stable
// https://github.com/rust-lang/rfcs/blob/master/text/1733-trait-alias.md
pub trait Value<Weight = Self>
where
    Self: Clone,
{
}
impl<T: Clone> Value for T {}

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

// TODO: Using generic associated types would give a cleaner interface and avoid boxing the iterators
// https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md
type Values<'a, V> = Box<dyn Iterator<Item = &'a V> + 'a>;
type Iter<'a, A, V> = Box<dyn Iterator<Item = Item<<A as Axis>::BinRange, &'a V>> + 'a>;

type ValuesMut<'a, V> = Box<dyn Iterator<Item = &'a mut V> + 'a>;
type IterMut<'a, A, V> = Box<dyn Iterator<Item = Item<<A as Axis>::BinRange, &'a mut V>> + 'a>;

pub trait Histogram<A: Axes, V>: Clone {
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

mod arrayhistogram;
pub use arrayhistogram::ArrayHistogram;
use num::One;
