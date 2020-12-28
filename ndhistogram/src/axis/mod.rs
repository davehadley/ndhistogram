use std::ops::Range;
use std::{collections::binary_heap::Iter, fmt::Display};

pub mod binrange;
mod uniform;
pub use uniform::Uniform;

pub trait Axis {
    type Coordinate;
    type BinRange;
    //type ItemIterator: Iterator<Item=(usize, Self::BinRange)>;

    fn index(&self, coordinate: Self::Coordinate) -> usize;
    fn numbins(&self) -> usize;
    fn size(&self) -> usize {
        self.numbins() + 2
    } // includes overflow and underflow

    fn bin(&self, index: usize) -> Option<Self::BinRange>;

    fn indices(&self) -> Box<dyn Iterator<Item = usize>> {
        Box::new(0..self.size())
    }

    fn items<'a>(&'a self) -> Box<dyn Iterator<Item = (usize, Option<Self::BinRange>)> + 'a> {
        Box::new(self.indices().map(move |it| (it, self.bin(it))))
    }
    fn bins<'a>(&'a self) -> Box<dyn Iterator<Item = Option<Self::BinRange>> + 'a> {
        Box::new(self.indices().map(move |it| self.bin(it)))
    }
}

// trait IterAxis: Axis {
//     type BinIterator;
//     fn iter_indices(&self) -> std::ops::Range<usize> {
//         0..self.size()
//     }
//     fn iter_bins(&self) -> Self::BinIterator;
// }

// impl <T> IntoIterator for T where T:Axis {
//     type Item = (usize, T::BinRange);
//     type IntoIter = ();

//     fn into_iter(self) -> Self::IntoIter {
//         self.iter_indices().map(|it| (it, self.bin(it)))
//     }
// }
