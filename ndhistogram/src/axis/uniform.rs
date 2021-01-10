use std::fmt::{Debug, Display};

use num_traits::Float;

use super::{Axis, BinInterval};

#[derive(Clone, PartialEq, Debug)]
pub struct Uniform<T = f64> {
    num: usize,
    low: T,
    high: T,
}

impl<T> Uniform<T>
where
    T: PartialOrd,
{
    pub fn new(num: usize, low: T, high: T) -> Uniform<T> {
        if num == 0 {
            panic!("Invalid axis num bins ({})", num);
        }
        if low >= high {
            panic!("Invalid axis range bins (low >= high)");
        }
        Uniform { num, low, high }
    }
}

impl<T> Uniform<T> {
    pub fn low(&self) -> &T {
        &self.low
    }
    pub fn high(&self) -> &T {
        &self.high
    }
}

// TODO: relax float retriction or add implementation for Integers
impl<T: Float> Axis for Uniform<T> {
    type Coordinate = T;
    type BinInterval = BinInterval<T>;

    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
        let frac = (*coordinate - self.low) / (self.high - self.low);
        if frac < T::zero() {
            return Some(0);
        } else if frac >= T::one() {
            return Some(self.num + 1);
        }
        let idx: T = T::from(self.num).unwrap() * frac;
        Some((idx.to_usize().unwrap()) + 1)
    }

    fn numbins(&self) -> usize {
        self.num + 2
    }

    fn bin(&self, index: usize) -> std::option::Option<<Self as Axis>::BinInterval> {
        if index == 0 {
            return Some(Self::BinInterval::underflow(self.low));
        } else if index == (self.num + 1) {
            return Some(Self::BinInterval::overflow(self.high));
        } else if index > (self.num + 1) {
            return None;
        }
        let start = (T::from(index - 1)?) * (self.high - self.low) / (T::from(self.num)?);
        let end = (T::from(index)?) * (self.high - self.low) / (T::from(self.num)?);
        Some(Self::BinInterval::new(start, end))
    }

    fn indices(&self) -> Box<dyn Iterator<Item = usize>> {
        Box::new(0..self.numbins())
    }
}

impl<T: Display> Display for Uniform<T> {
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

impl<'a, T: Float> IntoIterator for &'a Uniform<T> {
    type Item = (usize, <Uniform<T> as Axis>::BinInterval);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
