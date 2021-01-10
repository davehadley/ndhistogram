use std::{
    iter::Map,
    ops::{AddAssign, Index},
};

use super::{Fill, FillWeight, Histogram, Item, Iter, IterMut, Value, Values, ValuesMut};
use crate::{axes::Axes, axis::Axis};

#[derive(Debug, Clone)]
pub struct ArrayHistogram<A, V: Clone> {
    axes: A,
    values: Vec<V>,
}

impl<A: Axes, V: Default + Clone> ArrayHistogram<A, V> {
    pub fn new(axes: A) -> ArrayHistogram<A, V> {
        let size = axes.numbins();
        ArrayHistogram {
            axes,
            values: vec![V::default(); size],
        }
    }
}

impl<A: Axes, V: Value> Histogram<A, V> for ArrayHistogram<A, V> {
    fn value(&self, coordinate: &A::Coordinate) -> Option<&V> {
        let index = self.axes.index(coordinate)?;
        self.values.get(index)
    }

    fn axes(&self) -> &A {
        &self.axes
    }

    fn value_at_index(&self, index: usize) -> Option<&V> {
        self.values.get(index)
    }

    fn values<'a>(&'a self) -> Box<dyn Iterator<Item = &'a V> + 'a> {
        Box::new(self.values.iter())
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = Item<A::BinRange, &'a V>> + 'a> {
        Box::new(self.axes().iter().map(move |(index, binrange)| Item {
            index,
            bin: binrange,
            value: self.value_at_index(index).unwrap(),
        }))
    }

    fn value_at_index_mut(&mut self, index: usize) -> Option<&mut V> {
        self.values.get_mut(index)
    }

    fn values_mut(&mut self) -> ValuesMut<'_, V> {
        Box::new(self.values.iter_mut())
    }

    fn iter_mut(&mut self) -> IterMut<'_, A, V> {
        Box::new(
            self.axes
                .iter()
                .zip(self.values.iter_mut())
                .map(|((index, bin), value)| Item { index, bin, value }),
        )
    }
}

impl<'a, A: Axes, V: Value> IntoIterator for &'a ArrayHistogram<A, V> {
    type Item = Item<A::BinRange, &'a V>;

    type IntoIter = Iter<'a, A, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, A: Axes, V: Value + 'a> IntoIterator for &'a mut ArrayHistogram<A, V> {
    type Item = Item<A::BinRange, &'a mut V>;

    type IntoIter = IterMut<'a, A, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
