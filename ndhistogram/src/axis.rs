use std::collections::binary_heap::Iter;
use std::ops::Range;

pub trait Axis {
    type Coordinate;
    type BinItem;

    fn index(&self, coordinate: &Self::Coordinate) -> usize;
    fn numbins(&self) -> usize;
    fn size(&self) -> usize {
        self.numbins() + 2
    } // includes overflow and underflow

    fn bin(&self, index: usize) -> Option<&Self::BinItem>;
}

pub struct Uniform {
    num: usize,
    low: f64,
    high: f64,
}

impl Uniform {
    pub fn new(num: usize, low: f64, high: f64) -> Uniform {
        Uniform { num, low, high }
    }
}

impl Axis for Uniform {
    type Coordinate = f64;
    type BinItem = Range<Self::Coordinate>;

    fn index(&self, coordinate: &Self::Coordinate) -> usize {
        let frac = (coordinate - self.low) / (self.high - self.low);
        if frac < 0.0 {
            return 0;
        } else if frac >= 1.0 {
            return self.num + 1 as usize;
        }
        let idx = (self.num as f64) * frac;
        (idx as usize) + 1
    }

    fn numbins(&self) -> usize {
        self.num
    }

    fn bin(&self, _: usize) -> std::option::Option<&<Self as Axis>::BinItem> {
        todo!()
    }
}

trait IterAxis: Axis {
    type BinIterator;
    fn iter_indices(&self) -> std::ops::Range<usize> {
        0..self.size()
    }
    fn iter_bins(&self) -> Self::BinIterator;
}
