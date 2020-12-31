use std::{
    fmt::Display,
    ops::{AddAssign, Index},
};

use crate::{axes::Axes, axis::Axis};

pub trait AddOne {
    fn add_one(&mut self);
}
impl<T: One + AddAssign> AddOne for T {
    fn add_one(&mut self) {
        *self += Self::one();
    }
}

// TODO: Replace with trait alias when stable
// https://github.com/rust-lang/rfcs/blob/master/text/1733-trait-alias.md
pub trait Value<Weight = Self>
where
    Self: AddOne + AddAssign<Weight> + Clone,
{
}
impl<T: AddOne + AddAssign + Clone> Value for T {}

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

type Values<'a, V> = Box<dyn Iterator<Item = &'a V> + 'a>;
type Iter<'a, A, V> = Box<dyn Iterator<Item = Item<<A as Axis>::BinRange, &'a V>> + 'a>;

type ValuesMut<'a, V> = Box<dyn Iterator<Item = &'a mut V> + 'a>;
type IterMut<'a, A, V> = Box<dyn Iterator<Item = Item<<A as Axis>::BinRange, &'a mut V>> + 'a>;

pub trait Histogram<A: Axes, V>: Clone {
    fn axes(&self) -> &A;

    fn value_at_index(&self, index: usize) -> Option<&V>;

    fn value(&self, coordinate: A::Coordinate) -> Option<&V> {
        let index = self.axes().index(coordinate);
        self.value_at_index(index)
    }

    fn values(&self) -> Values<'_, V>;
    fn iter(&self) -> Iter<'_, A, V>;
}

pub trait Fill<A: Axes> {
    fn fill(&mut self, coordinate: A::Coordinate);
}

pub trait FillWeight<A: Axes, W> {
    fn fill_weight(&mut self, coordinate: A::Coordinate, weight: W);
}

//TODO: merge with histogram, I'm not sure that it makes sense for this to separate...
// although it makes development easier as iter_mut can be hard to implement...
// or it should be called something different like "DirectAccessHistogram"
pub trait MutableHistogram<A: Axes, V>: Histogram<A, V> {
    fn value_at_index_mut(&mut self, index: usize) -> Option<&mut V>;
    fn value_mut(&mut self, coordinate: A::Coordinate) -> Option<&mut V> {
        let index = self.axes().index(coordinate);
        self.value_at_index_mut(index)
    }

    fn values_mut(&mut self) -> ValuesMut<'_, V>;
    fn iter_mut(&mut self) -> IterMut<'_, A, V>;
}

mod arrayhistogram;
pub use arrayhistogram::ArrayHistogram;
use num::One;
