use super::axes::Axes;
use super::binvalues::BinValues;
pub struct Histogram<T: Axes, B: BinValues> {
    axes: T,
    bins: B,
}

impl<T: Axes, B: BinValues> Histogram<T, B> {
    pub fn new(axes: T, bins: B) -> Histogram<T, B> {
        Histogram { axes, bins }
    }
}

impl<T: Axes, B: BinValues> Histogram<T, B> {
    pub fn fill(&mut self, coordinate: &T::Coordinate, weight: B::Weight) {
        let index = self.axes.index(&coordinate);
        self.bins.fill(index, weight);
    }
}
