use super::axis::Axis;

pub trait Axes {
    type Coordinate;
    fn index(&self, coordinate: &Self::Coordinate) -> usize;
}

pub struct Axes1D<X: Axis> {
    x: X,
}

impl<X: Axis> Axes1D<X> {
    pub fn new(x: X) -> Axes1D<X> {
        Axes1D { x }
    }
}
pub struct Axes2D<X: Axis, Y: Axis> {
    x: X,
    y: Y,
}

impl<X: Axis, Y: Axis> Axes2D<X, Y> {
    pub fn new(x: X, y: Y) -> Axes2D<X, Y> {
        Axes2D { x, y }
    }
}

impl<X: Axis> Axes for Axes1D<X> {
    type Coordinate = X::Coordinate;

    fn index(&self, coordinate: &Self::Coordinate) -> usize {
        self.x.index(coordinate)
    }
}

impl<X: Axis, Y: Axis> Axes for Axes2D<X, Y> {
    type Coordinate = (X::Coordinate, Y::Coordinate);

    fn index(&self, coordinate: &Self::Coordinate) -> usize {
        let ix = self.x.index(&coordinate.0);
        let iy = self.y.index(&coordinate.1);
        ix + self.x.size() * iy
    }
}
