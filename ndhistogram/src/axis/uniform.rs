use num::{Float, FromPrimitive, NumCast};
use std::{
    fmt::{Debug, Display},
    iter::Map,
    ops::Mul,
    ops::Range,
    ops::{RangeBounds, RangeFrom, RangeTo, Sub},
};

use super::{binrange::ContinuousBinRange, Axis};

#[derive(Clone, PartialEq, Debug)]
pub struct Uniform<T = f64> {
    num: usize,
    low: T,
    high: T,
}

impl<T> Uniform<T>
where
    T: PartialOrd + Debug,
{
    pub fn new(num: usize, low: T, high: T) -> Uniform<T> {
        if num == 0 {
            panic!("Invalid axis num bins ({})", num);
        }
        if low >= high {
            panic!("Invalid axis range bins (low::{:?}, high:{:?})", low, high);
        }
        Uniform { num, low, high }
    }
}

impl<T: Float> Axis for Uniform<T> {
    type Coordinate = T;
    type BinRange = ContinuousBinRange<T>;

    fn index(&self, coordinate: Self::Coordinate) -> usize {
        let frac = (coordinate - self.low) / (self.high - self.low);
        if frac < T::zero() {
            return 0;
        } else if frac >= T::one() {
            return self.num + 1;
        }
        let idx: T = T::from(self.num).unwrap() * frac;
        (idx.to_usize().unwrap()) + 1
    }

    fn numbins(&self) -> usize {
        self.num + 2
    }

    fn bin(&self, index: usize) -> std::option::Option<<Self as Axis>::BinRange> {
        if index == 0 || (index - 1) >= self.num {
            return None;
        }
        // let start = ((index - 1) as f64) * (self.high - self.low) / (self.num as f64);
        // let end = (index as f64) * (self.high - self.low) / (self.num as f64);
        let start = (T::from(index - 1)?) * (self.high - self.low) / (T::from(self.num)?);
        let end = (T::from(index)?) * (self.high - self.low) / (T::from(self.num)?);
        Some(Self::BinRange::Bin { start, end })
    }

    fn indices(&self) -> Box<dyn Iterator<Item = usize>> {
        Box::new(0..self.numbins())
    }

    // fn items<'a>(&'a self) -> Box<dyn Iterator<Item = (usize, Option<Self::BinRange>)> + 'a> {
    //     Box::new(self.indices().map(move |it| (it, self.bin(it))))
    // }

    // fn bins<'a>(&'a self) -> Box<dyn Iterator<Item = Option<Self::BinRange>> + 'a> {
    //     Box::new(self.indices().map(move |it| self.bin(it)))
    // }
}

impl Display for Uniform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Axis{{# bins={}, range=[{}, {}), class={}}}",
            self.num,
            self.low,
            self.high,
            stringify!(Uniform)
        )
    }
}

impl<'a> IntoIterator for &'a Uniform {
    type Item = (usize, <Uniform as Axis>::BinRange);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
