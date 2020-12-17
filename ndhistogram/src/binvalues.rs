pub trait BinValues {
    type Value;
    type Weight;
    fn fill(&mut self, index: usize, weight: Self::Weight);
    fn get(&self, index: usize) -> Option<Self::Value>;
}
