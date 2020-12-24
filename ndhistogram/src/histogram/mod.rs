use crate::axes::Axes;
pub trait Histogram<A: Axes, V> {
    fn fill(&mut self, coordinate: &A::Coordinate);
    fn value(&self, coordinate: &A::Coordinate) -> Option<&V>;
}

mod arrayhistogram;
pub use arrayhistogram::ArrayHistogram;
