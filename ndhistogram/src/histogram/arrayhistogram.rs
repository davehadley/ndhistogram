use std::{iter::Map, ops::AddAssign};

use num::One;

use super::{Fill, FillWeight, Histogram, Item, MutableHistogram, Value};
use crate::axes::Axes;

#[derive(Debug, Clone)]
pub struct ArrayHistogram<A, V: Clone> {
    axes: A,
    values: Vec<V>,
}

impl<A: Axes, V: Default + Clone> ArrayHistogram<A, V> {
    pub fn new(axes: A) -> ArrayHistogram<A, V> {
        let size = axes.size();
        ArrayHistogram {
            axes,
            values: vec![V::default(); size],
        }
    }
}

impl<'a, A: Axes, V: 'a + Value> Histogram<'a, A, V> for ArrayHistogram<A, V> {
    type Values = std::slice::Iter<'a, V>;
    type Iter = Box<dyn Iterator<Item = Item<A::BinRange, &'a V>> + 'a>;

    fn value(&self, coordinate: A::Coordinate) -> Option<&V> {
        let index = self.axes.index(coordinate);
        self.values.get(index)
    }

    fn axes(&self) -> &A {
        &self.axes
    }

    fn value_at_index(&self, index: usize) -> Option<&V> {
        self.values.get(index)
    }

    fn values(&'a self) -> Self::Values {
        self.values.iter()
    }

    fn iter(&'a self) -> Self::Iter {
        Box::new(self.axes().iter().map(move |(index, binrange)| Item {
            index,
            bin: binrange,
            value: self.value_at_index(index).unwrap(),
        }))
    }
}

impl<A: Axes, V: Value> Fill<A> for ArrayHistogram<A, V> {
    fn fill(&mut self, coordinate: A::Coordinate) {
        let index = self.axes.index(coordinate);
        self.values[index] += V::one();
    }
}

impl<A: Axes, V: Value<W>, W> FillWeight<A, W> for ArrayHistogram<A, V> {
    fn fill_weight(&mut self, coordinate: A::Coordinate, weight: W) {
        let index = self.axes.index(coordinate);
        self.values[index] += weight;
    }
}

impl<'a, A: Axes, V: 'a + Value> MutableHistogram<'a, A, V> for ArrayHistogram<A, V> {
    type ValuesMut = std::slice::IterMut<'a, V>;
    type IterMut = Box<dyn Iterator<Item = Item<A::BinRange, &'a mut V>> + 'a>;

    fn value_at_index_mut(&mut self, index: usize) -> Option<&mut V> {
        self.values.get_mut(index)
    }

    fn values_mut(&'a mut self) -> Self::ValuesMut {
        self.values.iter_mut()
    }

    fn iter_mut(&'a mut self) -> Self::IterMut {
        Box::new(
            self.axes
                .iter()
                .zip(self.values.iter_mut())
                .map(|((index, bin), value)| Item { index, bin, value }),
        )
    }
}
