use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Range;

pub mod binrange;
mod uniform;
pub use uniform::Uniform;
mod category;
pub use category::Category;
mod noflow;
pub use noflow::NoFlow;

enum AxisIndexType {
    Overflow,
    Underflow,
    Bin,
}

struct AxisIndex {
    index: usize,
    index_type: AxisIndexType,
}

impl From<AxisIndex> for usize {
    fn from(index: AxisIndex) -> Self {
        index.index
    }
}

type Iter<'a, BinRange> = Box<dyn Iterator<Item = (usize, BinRange)> + 'a>;
type Indices = Box<dyn Iterator<Item = usize>>;
type Bins<'a, BinRange> = Box<dyn Iterator<Item = BinRange> + 'a>;

pub trait Axis: Clone {
    type Coordinate;
    type BinRange;

    fn index(&self, coordinate: &Self::Coordinate) -> Option<Index>;
    fn numbins(&self) -> usize;

    fn bin(&self, index: usize) -> Option<Self::BinRange>;

    fn indices(&self) -> Indices {
        Box::new(0..self.numbins())
    }

    fn iter(&self) -> Iter<'_, Self::BinRange> {
        Box::new(self.indices().map(move |it| (it, self.bin(it).unwrap())))
    }

    fn bins(&self) -> Bins<'_, Self::BinRange> {
        Box::new(self.indices().map(move |it| self.bin(it).unwrap()))
    }
}
