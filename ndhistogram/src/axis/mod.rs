use std::fmt::Debug;
use std::ops::Range;
use std::{collections::binary_heap::Iter, fmt::Display};

pub mod binrange;
mod uniform;
pub use uniform::Uniform;
mod category;
pub use category::Category;

pub trait Axis: Clone {
    type Coordinate;
    type BinRange;

    fn index(&self, coordinate: Self::Coordinate) -> usize;
    fn numbins(&self) -> usize;

    fn bin(&self, index: usize) -> Option<Self::BinRange>;

    fn indices(&self) -> Box<dyn Iterator<Item = usize>> {
        Box::new(0..self.numbins())
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (usize, Self::BinRange)> + 'a> {
        Box::new(self.indices().map(move |it| (it, self.bin(it).unwrap())))
    }

    fn bins<'a>(&'a self) -> Box<dyn Iterator<Item = Self::BinRange> + 'a> {
        Box::new(self.indices().map(move |it| self.bin(it).unwrap()))
    }
}
