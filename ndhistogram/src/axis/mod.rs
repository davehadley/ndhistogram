use std::ops::Range;
use std::{collections::binary_heap::Iter, fmt::Display};

pub mod binrange;
mod uniform;
pub use uniform::Uniform;

pub trait Axis: Clone {
    type Coordinate;
    type BinRange;

    fn index(&self, coordinate: Self::Coordinate) -> usize;
    fn numbins(&self) -> usize;
    fn size(&self) -> usize {
        self.numbins() + 2
    } // includes overflow and underflow

    fn bin(&self, index: usize) -> Option<Self::BinRange>;

    fn indices(&self) -> Box<dyn Iterator<Item = usize>> {
        Box::new(0..self.size())
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (usize, Option<Self::BinRange>)> + 'a> {
        Box::new(self.indices().map(move |it| (it, self.bin(it))))
    }
    fn bins<'a>(&'a self) -> Box<dyn Iterator<Item = Option<Self::BinRange>> + 'a> {
        Box::new(self.indices().map(move |it| self.bin(it)))
    }
}
