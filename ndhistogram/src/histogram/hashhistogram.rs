use std::collections::HashMap;

use super::histogram::{Histogram, Iter, IterMut, ValuesMut};
use crate::{axis::Axis, Item};

/// A sparse N-dimensional [Histogram] that stores its values in a [HashMap].
///
/// Only bins that are filled will consume memory.
/// This makes high-dimensional, many-binned (but mostly empty) histograms
///  possible. If memory usage is not a concern, see [VecHistogram].
///
/// See [sparsehistogram] for examples of its use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashHistogram<A, V> {
    axes: A,
    values: HashMap<usize, V>,
}

impl<A: Axis, V> HashHistogram<A, V> {
    /// Factory method for HashHistogram. It is recommended to use the
    /// [sparsehistogram](crate::sparsehistogram) macro instead.
    pub fn new(axes: A) -> HashHistogram<A, V> {
        HashHistogram {
            axes,
            values: HashMap::new(),
        }
    }
}

impl<A: Axis, V: Default> Histogram<A, V> for HashHistogram<A, V> {
    fn axes(&self) -> &A {
        &self.axes
    }

    fn value_at_index(&self, index: usize) -> Option<&V> {
        self.values.get(&index)
    }

    fn values(&self) -> super::histogram::Values<'_, V> {
        Box::new(self.values.values())
    }

    fn iter(&self) -> Iter<'_, A, V> {
        Box::new(self.values.iter().map(move |(index, value)| Item {
            index: *index,
            bin: self.axes.bin(*index).unwrap(),
            value,
        }))
    }

    fn value_at_index_mut(&mut self, index: usize) -> Option<&mut V> {
        self.values.get_mut(&index)
    }

    fn values_mut(&mut self) -> ValuesMut<'_, V> {
        Box::new(self.values.values_mut())
    }

    fn iter_mut(&mut self) -> IterMut<'_, A, V> {
        let axes = &self.axes;
        Box::new(self.values.iter_mut().map(move |(index, value)| Item {
            index: *index,
            bin: axes.bin(*index).unwrap(),
            value,
        }))
    }

    fn fill(&mut self, coordinate: &A::Coordinate)
    where
        V: crate::Fill,
    {
        if let Some(index) = self.axes.index(coordinate) {
            self.values.entry(index).or_default().fill();
        }
    }

    fn fill_with<D>(&mut self, coordinate: &A::Coordinate, data: D)
    where
        V: crate::FillWith<D>,
    {
        if let Some(index) = self.axes.index(coordinate) {
            self.values.entry(index).or_default().fill_with(data);
        }
    }

    fn fill_with_weighted<D, W>(&mut self, coordinate: &A::Coordinate, data: D, weight: W)
    where
        V: crate::FillWithWeighted<D, W>,
    {
        if let Some(index) = self.axes.index(coordinate) {
            self.values
                .entry(index)
                .or_default()
                .fill_with_weighted(data, weight);
        }
    }
}
