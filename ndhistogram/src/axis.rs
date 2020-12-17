pub trait Axis {
    type Coordinate;
    fn index(&self, coordinate: &Self::Coordinate) -> usize;
    fn size(&self) -> usize;
}
