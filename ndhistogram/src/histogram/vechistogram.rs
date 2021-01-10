use std::fmt::Display;

use crate::axes::Axes;

use super::histogram::{Histogram, Item, Iter, IterMut, ValuesMut};

#[derive(Debug, Clone)]
pub struct VecHistogram<A, V> {
    axes: A,
    values: Vec<V>,
}

impl<A: Axes, V: Default + Clone> VecHistogram<A, V> {
    pub fn new(axes: A) -> VecHistogram<A, V> {
        let size = axes.numbins();
        VecHistogram {
            axes,
            values: vec![V::default(); size],
        }
    }
}

impl<A: Axes, V> Histogram<A, V> for VecHistogram<A, V> {
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

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = Item<A::BinInterval, &'a V>> + 'a> {
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

impl<'a, A: Axes, V> IntoIterator for &'a VecHistogram<A, V> {
    type Item = Item<A::BinInterval, &'a V>;

    type IntoIter = Iter<'a, A, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, A: Axes, V: 'a> IntoIterator for &'a mut VecHistogram<A, V> {
    type Item = Item<A::BinInterval, &'a mut V>;

    type IntoIter = IterMut<'a, A, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<A: Axes, V> Display for VecHistogram<A, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VecHistogram{}D", self.axes().num_dim())
    }
}
