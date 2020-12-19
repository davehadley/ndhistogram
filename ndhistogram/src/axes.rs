use super::axis::Axis;

pub trait Axes {
    type Coordinate;
    fn index(&self, coordinate: &Self::Coordinate) -> usize;
}

impl<X: Axis> Axes for (X,) {
    type Coordinate = X::Coordinate;

    fn index(&self, coordinate: &Self::Coordinate) -> usize {
        self.0.index(coordinate)
    }
}

impl<X: Axis, Y: Axis> Axes for (X, Y) {
    type Coordinate = (X::Coordinate, Y::Coordinate);

    fn index(&self, coordinate: &Self::Coordinate) -> usize {
        let ix = self.0.index(&coordinate.0);
        let iy = self.1.index(&coordinate.1);
        ix + self.0.size() * iy
    }
}
