use crate::histogram::Grow;

use super::axis::Axis;

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

macro_rules! impl_axes {
    () => {
        pub trait Axes: Axis {}
    };
    ( ($index:tt => $type_parameter:ident), $( ($nth_index:tt => $nth_type_parameter:ident), )* ) => {
        impl<X: Axis> Axes for (X,) {}
        impl_axes!($(($nth_index => $nth_type_parameter), )*);
    };
}

impl_axes! {(0 => T),}

impl<X: Axis + Grow<<X as Axis>::Coordinate>> Grow<<Self as Axis>::Coordinate> for (X,) {
    fn grow(&mut self, newcoordinate: &<Self as Axis>::Coordinate) -> Result<(), ()> {
        self.0.grow(newcoordinate)
    }
}

impl<X: Axis + Grow<<X as Axis>::Coordinate>, Y: Axis + Grow<<Y as Axis>::Coordinate>>
    Grow<<Self as Axis>::Coordinate> for (X, Y)
{
    fn grow(&mut self, newcoordinate: &<Self as Axis>::Coordinate) -> Result<(), ()> {
        self.0.grow(&newcoordinate.0)?;
        self.1.grow(&newcoordinate.1)?;
        Ok(())
    }
}
