use super::{category::Value, Axis, Category, SingleValueBinInterval};
use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq, Debug)]
pub struct CategoryNoFlow<T>
where
    T: Value,
{
    axis: Category<T>,
}

impl<T: Value> CategoryNoFlow<T> {
    pub fn new<I: IntoIterator<Item = T>>(values: I) -> Self {
        Self {
            axis: Category::new(values),
        }
    }
}

impl<T: Value> Axis for CategoryNoFlow<T> {
    type Coordinate = T;
    type BinRange = SingleValueBinInterval<T>;

    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
        let index = self.axis.index(coordinate)?;
        if index == self.axis.numbins() - 1 {
            return None;
        }
        Some(index)
    }

    fn numbins(&self) -> usize {
        self.axis.numbins() - 1
    }

    fn bin(&self, index: usize) -> Option<Self::BinRange> {
        let bin = self.axis.bin(index)?;
        match bin {
            SingleValueBinInterval::Overflow => None,
            SingleValueBinInterval::Bin { value: _ } => Some(bin),
        }
    }
}

impl<'a, T: Value> IntoIterator for &'a CategoryNoFlow<T> {
    type Item = (usize, <Category<T> as Axis>::BinRange);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Display + Value> Display for CategoryNoFlow<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.axis)
    }
}
