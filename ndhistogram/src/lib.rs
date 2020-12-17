use std::{collections::btree_set::Union, ops::AddAssign};

pub trait Axis {
    type Position;
    fn numbins(&self) -> usize;
    fn find_index(&self, position: &Self::Position) -> usize;
}

struct Uniform {
    numbins: usize,
    low: f64,
    high: f64,
}

impl Uniform {
    fn new(numbins: usize, low: f64, high: f64) -> Uniform {
        Uniform { numbins, low, high }
    }
}
//struct Variable { binedges : Vec<f64> },
//struct Category { values: Vec<String> }

impl Axis for Uniform {
    type Position = f64;
    fn numbins(&self) -> usize {
        self.numbins
    }

    fn find_index(&self, position: &f64) -> usize {
        0
    }
}

pub struct Histogram1D<T: Axis, U = f64> {
    axis: T,
    values: Vec<U>,
}

impl<T: Axis, U: AddAssign<f64> + Default + Clone> Histogram1D<T, U> {
    pub fn new(axis: T) -> Histogram1D<T, U> {
        let numbins = axis.numbins();
        Histogram1D {
            axis: axis,
            values: vec![U::default(); numbins],
        }
    }

    pub fn fill(&mut self, x: &T::Position) {
        let index = self.axis.find_index(x);
        let v = self.values.get_mut(index).unwrap();
        *v += 1.0;
    }
}

#[cfg(test)]
mod unittests;
