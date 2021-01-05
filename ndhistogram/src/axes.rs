use super::axis::Axis;

pub trait Axes: Axis {}

impl<X: Axis> Axes for (X,) {}

impl<X: Axis> Axis for (X,) {
    type Coordinate = X::Coordinate;
    type BinRange = X::BinRange;

    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
        self.0.index(coordinate)
    }

    fn numbins(&self) -> usize {
        self.0.numbins()
    }

    fn bin(&self, index: usize) -> Option<Self::BinRange> {
        self.0.bin(index)
    }
}

impl<X: Axis, Y: Axis> Axes for (X, Y) {}

impl<X: Axis, Y: Axis> Axis for (X, Y) {
    type Coordinate = (X::Coordinate, Y::Coordinate);
    type BinRange = (X::BinRange, Y::BinRange);

    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
        let ix = self.0.index(&coordinate.0)?;
        let iy = self.1.index(&coordinate.1)?;
        Some(ix + self.0.numbins() * iy)
    }

    fn numbins(&self) -> usize {
        self.0.numbins() * self.1.numbins()
    }

    fn bin(&self, index: usize) -> Option<Self::BinRange> {
        let ix = index % self.0.numbins();
        let iy = index / self.0.numbins();

        let bx = self.0.bin(ix)?;
        let by = self.1.bin(iy)?;
        Some((bx, by))
    }
}
