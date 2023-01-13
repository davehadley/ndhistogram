use std::hash::Hash;
use std::{collections::HashMap, fmt::Display};

use super::Axis;
use super::SingleValueBinInterval;

// Type-bound alias
pub trait Value: Eq + Hash + Clone {}
impl<T: Eq + Hash + Clone> Value for T {}

/// An axis to represent a set of discrete values or categories with an overflow bin.
///
/// This axis also includes an overflow bin, to include "other" values not given
/// when the axis was constructed.
/// See [CategoryNoFlow](crate::axis::CategoryNoFlow) for a variant that includes no overflow bin.
///
/// # Example
///
/// ```rust
/// use ndhistogram::axis::{Axis, Category, SingleValueBinInterval};
/// let colors = Category::new(vec!["red", "blue", "pink", "yellow", "black"]);
/// assert_eq!(colors.index(&"red"), Some(0));
/// assert_eq!(colors.index(&"green"), Some(5));
/// assert_eq!(colors.bin(1), Some(SingleValueBinInterval::new("blue")));
/// assert_eq!(colors.bin(5), Some(SingleValueBinInterval::overflow()));
/// ```
#[derive(Default, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Category<T>
where
    T: Eq + Hash,
{
    map_t_to_index: HashMap<T, usize>,
    map_index_to_t: HashMap<usize, T>,
}

impl<T: Value> Category<T> {
    fn insert(&mut self, value: T) {
        // TODO: implement solution that does not require storing two copies of T
        let index = self.map_index_to_t.len();
        self.map_index_to_t.insert(index, value.clone());
        self.map_t_to_index.insert(value, index);
    }

    fn get_index(&self, value: &T) -> Option<usize> {
        self.map_t_to_index.get(value).copied()
    }

    fn get_value(&self, index: usize) -> Option<&T> {
        self.map_index_to_t.get(&index)
    }

    fn len(&self) -> usize {
        self.map_index_to_t.len()
    }

    fn constructor<I: IntoIterator<Item = T>>(values: I) -> Self {
        let mut cat = Self {
            map_t_to_index: HashMap::new(),
            map_index_to_t: HashMap::new(),
        };
        // TODO: is it faster to directly construct the hashmap rather than repeatedly insert?
        values.into_iter().for_each(|it| cat.insert(it));
        cat
    }

    /// Factory method to create a category axis without an overflow bin.
    ///
    /// Takes an iterator over a set of values that represent each category.
    /// All other values will be mapped to the overflow bin.
    pub fn new<I: IntoIterator<Item = T>>(values: I) -> Self {
        Self::constructor(values)
    }
}

impl<T: Value> Axis for Category<T> {
    type Coordinate = T;

    type BinInterval = SingleValueBinInterval<T>;

    #[inline]
    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
        self.get_index(coordinate).or_else(|| Some(self.len()))
    }

    fn num_bins(&self) -> usize {
        self.len() + 1
    }

    fn bin(&self, index: usize) -> Option<Self::BinInterval> {
        let value = self.get_value(index);
        match value {
            Some(value) => Some(Self::BinInterval::new(value.clone())),
            None => {
                if index == self.len() {
                    Some(Self::BinInterval::overflow())
                } else {
                    None
                }
            }
        }
    }
}

impl<T: Display + Value> Display for Category<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let comma_separated_list = self
            .bins()
            .take(10)
            .map(|it| it.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{{{}}}", comma_separated_list)
    }
}

impl<'a, T: Value> IntoIterator for &'a Category<T> {
    type Item = (usize, <Category<T> as Axis>::BinInterval);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
