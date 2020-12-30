use std::{fmt::Display, ops::AddAssign};

use crate::axes::Axes;

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

#[derive(Debug)]
pub struct Item<T, V> {
    pub index: usize,
    pub bin: Option<T>,
    pub value: V,
}

pub trait Histogram<'a, A: Axes, V: 'a + Value>: Clone {
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

pub trait MutableHistogram<'a, A: Axes, V: 'a + Value>: Histogram<'a, A, V> {
    type ValuesMut: Iterator<Item = &'a mut V>;
    type IterMut: Iterator<Item = Item<A::BinRange, &'a mut V>>;

    fn value_at_index_mut(&mut self, index: usize) -> Option<&mut V>;
    fn values_mut(&'a mut self) -> Self::ValuesMut;
    fn iter_mut(&'a mut self) -> Self::IterMut;
}

mod arrayhistogram;
pub use arrayhistogram::ArrayHistogram;
use num::One;
