
trait Axis {
    type Coordinate;
    fn index(&self, coordinate: &Self::Coordinate) -> usize;
}

pub trait Axes {
    type Coordinate;
    fn index(&self, coordinate: &Self::Coordinate) -> usize;
}

pub trait BinStore {
    type Value;
    type Weight;
    fn fill(&mut self, index: usize, weight: Self::Weight);
    fn get(&self, index: usize) -> Option<Self::Value>;
}

pub struct Histogram<T: Axes, B: BinStore> {
    axes: T,
    bins: B 
}

impl <T: Axes, B: BinStore> Histogram<T, B> {
    pub fn fill(&mut self, coordinate: &T::Coordinate, weight: B::Weight) {
        let index = self.axes.index(&coordinate);
        self.bins.fill(index, weight);
    }
}

#[cfg(test)]
mod unittests;

