use super::{category::Value, Axis, Category, SingleValueBinInterval};
use std::fmt::{Debug, Display};

/// An axis to represent a finite set of discrete values or categories without an overflow bin.
///
/// Similar to [Category], however, no overflow bin is included.
///
/// # Example
///
/// ```rust
/// use ndhistogram::axis::{Axis, CategoryNoFlow, SingleValueBinInterval};
/// let colors = CategoryNoFlow::new(vec!["red", "blue", "pink", "yellow", "black"]);
/// assert_eq!(colors.index(&"red"), Some(0));
/// assert_eq!(colors.index(&"green"), None);
/// assert_eq!(colors.bin(1), Some(SingleValueBinInterval::new("blue")));
/// assert_eq!(colors.bin(5), None);
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct CategoryNoFlow<T>
where
    T: Value,
{
    axis: Category<T>,
}

impl<T: Value> CategoryNoFlow<T> {
    /// Factory method to create a category axis without an overflow bin.
    ///
    /// Takes a set of values that represent each category.
    /// All other values will not be included in this axis.
    pub fn new<I: IntoIterator<Item = T>>(values: I) -> Self {
        Self {
            axis: Category::new(values),
        }
    }
}

impl<T: Value> Axis for CategoryNoFlow<T> {
    type Coordinate = T;
    type BinInterval = SingleValueBinInterval<T>;

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

    fn bin(&self, index: usize) -> Option<Self::BinInterval> {
        let bin = self.axis.bin(index)?;
        match bin {
            SingleValueBinInterval::Overflow => None,
            SingleValueBinInterval::Bin { value: _ } => Some(bin),
        }
    }
}

impl<'a, T: Value> IntoIterator for &'a CategoryNoFlow<T> {
    type Item = (usize, <Category<T> as Axis>::BinInterval);
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
