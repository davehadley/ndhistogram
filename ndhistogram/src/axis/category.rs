use std::collections::HashMap;
use std::hash::Hash;

use super::binrange::SingleValuedBinRange;
use super::Axis;

// Type-bound alias
pub trait Value: Eq + Hash + Clone {}
impl<T: Eq + Hash + Clone> Value for T {}

#[derive(Debug, Clone, Default)]
pub struct Category<T> {
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

    pub fn new<I: IntoIterator<Item = T>>(values: I) -> Category<T> {
        let mut cat = Category {
            map_t_to_index: HashMap::new(),
            map_index_to_t: HashMap::new(),
        };
        // TODO: is it faster to directly construct the hashmap rather than repeatedly insert?
        values.into_iter().for_each(|it| cat.insert(it));
        cat
    }
}

impl<T: Value> Axis for Category<T> {
    type Coordinate = T;

    type BinRange = SingleValuedBinRange<T>;

    fn index(&self, coordinate: Self::Coordinate) -> Option<usize> {
        self.get_index(&coordinate).or_else(|| Some(self.len()))
    }

    fn numbins(&self) -> usize {
        self.len() + 1
    }

    fn bin(&self, index: usize) -> Option<Self::BinRange> {
        let value = self.get_value(index);
        match value {
            Some(value) => Some(Self::BinRange::new(value.clone())),
            None => {
                if index == self.len() {
                    Some(Self::BinRange::overflow())
                } else {
                    None
                }
            }
        }
    }
}
