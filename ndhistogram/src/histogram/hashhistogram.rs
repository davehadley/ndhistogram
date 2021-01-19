use std::collections::HashMap;

use super::histogram::{Histogram, Iter, IterMut, ValuesMut};
use crate::axis::Axis;

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

impl<A: Axis, V> Histogram<A, V> for HashHistogram<A, V> {
    fn axes(&self) -> &A {
        todo!()
    }

    fn value_at_index(&self, index: usize) -> Option<&V> {
        todo!()
    }

    fn values(&self) -> super::histogram::Values<'_, V> {
        todo!()
    }

    fn iter(&self) -> Iter<'_, A, V> {
        todo!()
    }

    fn value_at_index_mut(&mut self, index: usize) -> Option<&mut V> {
        todo!()
    }

    fn values_mut(&mut self) -> ValuesMut<'_, V> {
        todo!()
    }

    fn iter_mut(&mut self) -> IterMut<'_, A, V> {
        todo!()
    }
}
