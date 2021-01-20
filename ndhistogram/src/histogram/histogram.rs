use crate::{axis::Axis, FillWith};

use super::fill::{Fill, FillWithWeighted};

// TODO: Using generic associated types would give a cleaner interface and avoid boxing the iterators
// https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md
pub(crate) type Values<'a, V> = Box<dyn Iterator<Item = &'a V> + 'a>;
pub(crate) type Iter<'a, A, V> =
    Box<dyn Iterator<Item = Item<<A as Axis>::BinInterval, &'a V>> + 'a>;

pub(crate) type ValuesMut<'a, V> = Box<dyn Iterator<Item = &'a mut V> + 'a>;
pub(crate) type IterMut<'a, A, V> =
    Box<dyn Iterator<Item = Item<<A as Axis>::BinInterval, &'a mut V>> + 'a>;

/// A common interface for an ND histograms.
///
/// Implementations of this trait should handle storing the histogram bin values
/// and provide methods to fill and read those values.
///
/// The most commonly used implementation is [VecHistogram](crate::VecHistogram).
/// See [ndhistogram] for examples of its use.
pub trait Histogram<A: Axis, V> {
    /// The histogram [Axes](crate::Axes) that map coordinates to bin numbers.
    fn axes(&self) -> &A;

    /// Read a bin value given an index.
    /// Return an Option as the given index may not be valid for this histogram.
    fn value_at_index(&self, index: usize) -> Option<&V>;

    /// Read a bin value given a coordinate.
    /// Returns an Option as the given coordinate may not be mapped to a bin.
    fn value(&self, coordinate: &A::Coordinate) -> Option<&V> {
        let index = self.axes().index(coordinate)?;
        self.value_at_index(index)
    }

    /// Iterator over bin values.
    fn values(&self) -> Values<'_, V>;

    /// Iterator over bin indices, bin interval and bin values.
    fn iter(&self) -> Iter<'_, A, V>;

    /// Mutable access to a bin value at a given index.
    fn value_at_index_mut(&mut self, index: usize) -> Option<&mut V>;

    /// Mutable access to a bin value at a given coordinate.
    fn value_mut(&mut self, coordinate: &A::Coordinate) -> Option<&mut V> {
        let index = self.axes().index(coordinate)?;
        self.value_at_index_mut(index)
    }

    /// Mutable iterator over bin values.
    fn values_mut(&mut self) -> ValuesMut<'_, V>;
    /// Mutable iterator over bin indices, bin interval and bin values.
    fn iter_mut(&mut self) -> IterMut<'_, A, V>;

    /// Fill the histogram bin value at coordinate with unit weight.
    /// If the [Axes](crate::Axes) do not cover that coordinate, do nothing.
    /// See [Fill](crate::Fill).
    fn fill(&mut self, coordinate: &A::Coordinate)
    where
        V: Fill,
    {
        if let Some(value) = self.value_mut(coordinate) {
            value.fill()
        }
    }

    /// Fill the histogram bin value at coordinate with some data.
    /// If the [Axes](crate::Axes) do not cover that coordinate, do nothing.
    /// See [FillWith](crate::FillWith).
    fn fill_with<D>(&mut self, coordinate: &A::Coordinate, data: D)
    where
        V: FillWith<D>,
    {
        if let Some(value) = self.value_mut(coordinate) {
            value.fill_with(data)
        }
    }

    /// Fill the histogram bin value at coordinate with some data.
    /// If the [Axes](crate::Axes) do not cover that coordinate, do nothing.
    /// See [FillWithWeighted].
    fn fill_with_weighted<D, W>(&mut self, coordinate: &A::Coordinate, data: D, weight: W)
    where
        V: FillWithWeighted<D, W>,
    {
        if let Some(value) = self.value_mut(coordinate) {
            value.fill_with_weighted(data, weight)
        }
    }
}

/// Struct to be returned when iterating over [Histogram]s bins.
#[derive(Copy, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Item<T, V> {
    /// Bin number
    pub index: usize,
    /// Bin interval. See [Axis::BinInterval].
    pub bin: T,
    /// Bin value.
    pub value: V,
}

impl<T, V> Item<T, V> {
    /// Factory method to create [Item].
    pub fn new(index: usize, bin: T, value: V) -> Self {
        Self { index, bin, value }
    }
}
