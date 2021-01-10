use std::hash::Hash;
use std::{collections::HashMap, fmt::Display};

use super::binrange::SingleValuedBinRange;
use super::Axis;

// Type-bound alias
pub trait Value: Eq + Hash + Clone {}
impl<T: Eq + Hash + Clone> Value for T {}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Category<T>
where
    T: Value,
{
    map_t_to_index: HashMap<T, usize>,
    map_index_to_t: HashMap<usize, T>,
    // TODO: use const generics when stable https://rust-lang.github.io/rfcs/2000-const-generics.html
    isgrow: bool,
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

    fn constructor<I: IntoIterator<Item = T>>(values: I, grow: bool) -> Self {
        let mut cat = Self {
            map_t_to_index: HashMap::new(),
            map_index_to_t: HashMap::new(),
            isgrow: grow,
        };
        // TODO: is it faster to directly construct the hashmap rather than repeatedly insert?
        values.into_iter().for_each(|it| cat.insert(it));
        cat
    }

    pub fn new<I: IntoIterator<Item = T>>(values: I) -> Self {
        Self::constructor(values, false)
    }

    pub fn growable<I: IntoIterator<Item = T>>(values: I) -> Self {
        Self::constructor(values, true)
    }
}

impl<T: Value> Axis for Category<T> {
    type Coordinate = T;

    type BinRange = SingleValuedBinRange<T>;

    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
        self.get_index(&coordinate)
            .or_else(|| if self.isgrow { None } else { Some(self.len()) })
    }

    fn numbins(&self) -> usize {
        if self.isgrow {
            self.len()
        } else {
            self.len() + 1
        }
    }

    fn bin(&self, index: usize) -> Option<Self::BinRange> {
        let value = self.get_value(index);
        match value {
            Some(value) => Some(Self::BinRange::new(value.clone())),
            None => {
                if index == self.len() && !self.isgrow {
                    Some(Self::BinRange::overflow())
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
    type Item = (usize, <Category<T> as Axis>::BinRange);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
