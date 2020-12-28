use std::{iter::Map, ops::AddAssign};

use num::One;

use super::{Fill, FillWeight, Histogram, Item, ItemMut, MutableHistogram};
use crate::axes::Axes;

#[derive(Debug)]
pub struct ArrayHistogram<A, V> {
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

impl<'a, A: Axes, V: One + AddAssign + 'a> Histogram<'a, A, V> for ArrayHistogram<A, V> {
    type Values = std::slice::Iter<'a, V>;
    type Iter = Box<dyn Iterator<Item = Item<'a, A::BinRange, V>> + 'a>;

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

impl<A: Axes, V: One + AddAssign> Fill<A> for ArrayHistogram<A, V> {
    fn fill(&mut self, coordinate: A::Coordinate) {
        let index = self.axes.index(coordinate);
        self.values[index] += V::one();
    }
}

impl<A: Axes, V: AddAssign<W>, W> FillWeight<A, W> for ArrayHistogram<A, V> {
    fn fill_weight(&mut self, coordinate: A::Coordinate, weight: W) {
        let index = self.axes.index(coordinate);
        self.values[index] += weight;
    }
}

impl<'a, A: Axes, V: One + AddAssign + 'a> MutableHistogram<'a, A, V>
    for ArrayHistogram<A, V>
{
    type ValuesMut = std::slice::IterMut<'a, V>;
    type IterMut = Box<dyn Iterator<Item = ItemMut<'a, A::BinRange, V>> + 'a>;
    //type IterMut = Box<dyn Iterator<Item = ItemMut<'a, A::BinRange, V>>>;

    fn value_at_index_mut(&mut self, index: usize) -> Option<&mut V> {
        self.values.get_mut(index)
    }

    fn values_mut(&'a mut self) -> Self::ValuesMut {
        self.values.iter_mut()
    }

    fn iter_mut(&'a mut self) -> Self::IterMut {
        Box::new(
        TestIterMut { 
            axesiter: self.axes.iter(),
            values: &mut self.values 
        }
    )
    }
}

struct TestIterMut<'a, A, V> {
    axesiter: A,
    values: &'a mut [V],
}

impl <'a, A: Iterator<Item=(usize, Option<Z>)>, V, Z> Iterator for TestIterMut<'a, A, V> {
    type Item = ItemMut<'a, Z, V>;

    fn next(&mut self) -> Option<Self::Item> {
        let (index, bin) = self.axesiter.next()?;
        let value = self.values.get_mut(index)?;
        Some(ItemMut { index: index, bin: bin, value: value })
    }
}
