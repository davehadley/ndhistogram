use std::{
    collections::HashMap,
    collections::HashSet,
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use super::histogram::{Histogram, Iter, IterMut, ValuesMut};
use crate::{axis::Axis, Item};

/// A sparse N-dimensional [Histogram] that stores its values in a [HashMap].
///
/// Only bins that are filled will consume memory.
/// This makes high-dimensional, many-binned (but mostly empty) histograms
///  possible. If memory usage is not a concern, see [VecHistogram](crate::VecHistogram).
///
/// See [sparsehistogram] for examples of its use.
#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct HashHistogram<A, V> {
    axes: A,
    values: HashMap<usize, V>,
}

impl<A: Axis, V> HashHistogram<A, V> {
    /// Factory method for HashHistogram. It is recommended to use the
    /// [sparsehistogram](crate::sparsehistogram) macro instead.
    pub fn new(axes: A) -> Self {
        Self {
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

impl<'a, A: Axis, V> IntoIterator for &'a HashHistogram<A, V>
where
    HashHistogram<A, V>: Histogram<A, V>,
{
    type Item = Item<A::BinInterval, &'a V>;

    type IntoIter = Iter<'a, A, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, A: Axis, V: 'a> IntoIterator for &'a mut HashHistogram<A, V>
where
    HashHistogram<A, V>: Histogram<A, V>,
{
    type Item = Item<A::BinInterval, &'a mut V>;

    type IntoIter = IterMut<'a, A, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<A: Axis, V> Display for HashHistogram<A, V>
where
    HashHistogram<A, V>: Histogram<A, V>,
    V: Clone + Into<f64>,
    A::BinInterval: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sum = self
            .values()
            .map(|it| {
                let x: f64 = it.clone().into();
                x
            })
            .fold(0.0, |it, value| it + value);
        write!(
            f,
            "HashHistogram{}D({} bins, sum={})",
            self.axes().num_dim(),
            self.axes().num_bins(),
            sum
        )?;
        Ok(())
    }
}

macro_rules! impl_binary_op {
    ($Trait:tt, $method:tt, $mathsymbol:tt) => {
        impl<A: Axis + PartialEq + Clone, V> $Trait<&HashHistogram<A, V>> for &HashHistogram<A, V>
        where
            HashHistogram<A, V>: Histogram<A, V>,
            V: Clone + Default,
            for<'a> &'a V: $Trait<Output = V>,
        {
            type Output = Result<HashHistogram<A, V>, ()>;

            fn $method(self, rhs: &HashHistogram<A, V>) -> Self::Output {
                if self.axes() != rhs.axes() {
                    return Err(());
                }
                let indices: HashSet<usize> = self.values.keys().chain(rhs.values.keys()).copied().collect();
                let values: HashMap<usize, V> = indices.into_iter().map(|index| {
                    let left = self.values.get(&index);
                    let right = rhs.values.get(&index);
                    let newvalue = match (left, right) {
                        (Some(left), Some(right)) => { left $mathsymbol right },
                        (None, Some(right)) => { &V::default() $mathsymbol right },
                        (Some(left), None) => { left $mathsymbol &V::default() },
                        (None, None) => { unreachable!() },
                    };
                    (index, newvalue)
                }).collect();
                Ok(HashHistogram {
                    axes: self.axes().clone(),
                    values,
                })
            }
        }
    };
}

impl_binary_op! {Add, add, +}
impl_binary_op! {Sub, sub, -}
impl_binary_op! {Mul, mul, *}
impl_binary_op! {Div, div, /}
