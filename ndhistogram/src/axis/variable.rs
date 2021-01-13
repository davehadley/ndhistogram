use std::fmt::Display;

use super::{Axis, BinInterval};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable<T = f64> {
    bin_edges: Vec<T>,
}

impl<T> Variable<T>
where
    T: PartialOrd + Copy,
{
    pub fn new<I: IntoIterator<Item = T>>(bin_edges: I) -> Self {
        let mut bin_edges: Vec<T> = bin_edges.into_iter().collect();
        if bin_edges.len() < 2 {
            panic!("Invalid axis number of bin edges ({})", bin_edges.len());
        }
        bin_edges.sort_by(|a, b| a.partial_cmp(b).expect("failed to sort bin_edges."));
        Self { bin_edges }
    }

    /// Low edge of axis (excluding underflow bin).
    pub fn low(&self) -> &T {
        &self
            .bin_edges
            .first()
            .expect("Variable binedges unexpectedly empty")
    }

    /// High edge of axis (excluding overflow bin).
    pub fn high(&self) -> &T {
        &self
            .bin_edges
            .last()
            .expect("Variable binedges unexpectedly empty")
    }
}

impl<T> Axis for Variable<T>
where
    T: PartialOrd + Copy,
{
    type Coordinate = T;
    type BinInterval = BinInterval<T>;

    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
        match self.bin_edges.binary_search_by(|probe| {
            probe
                .partial_cmp(&coordinate)
                .expect("incomparable values. NAN bin edges?")
        }) {
            Ok(index) => Some(index + 1),
            Err(index) => Some(index),
        }
    }

    fn numbins(&self) -> usize {
        self.bin_edges.len() + 1
    }

    fn bin(&self, index: usize) -> Option<Self::BinInterval> {
        if index == 0 {
            Some(Self::BinInterval::underflow(*self.low()))
        } else if index == self.bin_edges.len() {
            Some(Self::BinInterval::overflow(*self.high()))
        } else if index < self.bin_edges.len() {
            Some(Self::BinInterval::new(
                self.bin_edges[index - 1],
                self.bin_edges[index],
            ))
        } else {
            None
        }
    }
}

impl<T: Display + PartialOrd + Copy> Display for Variable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Axis{{# bins={}, range=[{}, {}), class={}}}",
            self.bin_edges.len() - 1,
            self.low(),
            self.high(),
            stringify!(Variable)
        )
    }
}

impl<'a, T> IntoIterator for &'a Variable<T>
where
    Variable<T>: Axis,
{
    type Item = (usize, <Variable<T> as Axis>::BinInterval);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
