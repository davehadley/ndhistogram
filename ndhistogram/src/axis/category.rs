use std::collections::HashMap;
use std::hash::Hash;

use super::binrange::SingleValuedBinRange;
use super::Axis;

// Type-bound alias
pub trait Value: Eq + Hash + Clone {}
impl<T: Eq + Hash + Clone> Value for T {}

#[derive(Debug, Clone)]
pub struct Category<T> {
    map: HashMap<T, usize>,
}

impl<T: Value> Category<T> {
    pub fn new<I: IntoIterator<Item = T>>(values: I) -> Category<T> {
        Category {
            map: values
                .into_iter()
                .enumerate()
                .map(|(index, value)| (value, index))
                .collect(),
        }
    }
}

impl<T: Value> Axis for Category<T> {
    type Coordinate = T;

    type BinRange = SingleValuedBinRange<T>;

    fn index(&self, coordinate: Self::Coordinate) -> Option<usize> {
        Some(*(self.map.get(&coordinate)?))
    }

    fn numbins(&self) -> usize {
        self.map.len() + 1
    }

    fn bin(&self, index: usize) -> Option<Self::BinRange> {
        todo!()
    }
}
