use std::ops::AddAssign;

pub trait BinValues {
    type Value;
    type Weight;
    fn fill(&mut self, index: usize, weight: Self::Weight);
    fn get(&self, index: usize) -> Option<&Self::Value>;
}

pub struct VecBinValues<T = f64> {
    data: Vec<T>,
}

impl<T: Default + Clone> VecBinValues<T> {
    pub fn new(size: usize) -> VecBinValues<T> {
        VecBinValues {
            data: vec![T::default(); size],
        }
    }
}

impl<T: AddAssign> BinValues for VecBinValues<T> {
    type Value = T;
    type Weight = T;

    fn fill(&mut self, index: usize, weight: Self::Weight) {
        let binvalue = self.data.get_mut(index).unwrap();
        *binvalue += weight;
    }

    fn get(&self, index: usize) -> Option<&Self::Value> {
        self.data.get(index)
    }
}
