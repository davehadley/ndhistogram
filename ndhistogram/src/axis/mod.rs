use std::ops::Range;
use std::{collections::binary_heap::Iter, fmt::Display};

mod uniform;
pub use uniform::Uniform;

pub trait Axis {
    type Coordinate;
    type BinRange;

    fn index(&self, coordinate: &Self::Coordinate) -> usize;
    fn numbins(&self) -> usize;
    fn size(&self) -> usize {
        self.numbins() + 2
    } // includes overflow and underflow

    fn bin(&self, index: usize) -> Option<Self::BinRange>;
}

// trait IterAxis: Axis {
//     type BinIterator;
//     fn iter_indices(&self) -> std::ops::Range<usize> {
//         0..self.size()
//     }
//     fn iter_bins(&self) -> Self::BinIterator;
// }
