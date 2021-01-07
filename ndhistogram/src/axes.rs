use crate::histogram::Grow;

use super::axis::Axis;

pub trait Axes: Axis {
    fn num_dim(&self) -> usize;
}

// impl<X: Axis> Axis for (X,) {
//     type Coordinate = X::Coordinate;
//     type BinRange = X::BinRange;

//     fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
//         self.0.index(coordinate)
//     }

//     fn numbins(&self) -> usize {
//         self.0.numbins()
//     }

//     fn bin(&self, index: usize) -> Option<Self::BinRange> {
//         self.0.bin(index)
//     }
// }

impl<X: Axis, Y: Axis> Axes for (X, Y) {
    fn num_dim(&self) -> usize {
        2
    }
}

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

// from the little book of rust macros <https://danielkeep.github.io/tlborm/book/blk-counting.html>
macro_rules! count_idents {
    ($($idents:ident),* $(,)*) => {
        {
            #[allow(dead_code, non_camel_case_types)]
            enum Idents { $($idents,)* __CountIdentsLast }
            const COUNT: usize = Idents::__CountIdentsLast as usize;
            COUNT
        }
    };
}

macro_rules! next_ntuple_impl {
    ($name:ident, $($other:ident,)*) => (ntuple_impl! { $($other,)* })
}

macro_rules! ntuple_coordinate {
    ( $name:ident ) => { <$name as Axis>::Coordinate };
    ( $($name:ident),+$(,)* ) => { ($(<$name as Axis>::Coordinate),*) };
}

macro_rules! ntuple_binrange {
    ( $name:ident ) => { <$name as Axis>::BinRange };
    ( $($name:ident),+$(,)* ) => { ($(<$name as Axis>::BinRange),*) };
}

macro_rules! ntuple_impl {
    () => ();
    ( $($name:ident),+$(,)* )  => {

        impl<$($name:Axis),*> Axes for ($($name),* ,) {
            fn num_dim(&self) -> usize {
                count_idents!($($name),*)
            }
        }

        impl<$($name:Axis),*> Axis for ($($name),* ,) {
            type Coordinate = ntuple_coordinate!($($name),*);
            type BinRange = ntuple_binrange!($($name),*);

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
        next_ntuple_impl!($($name,)+);
    };
}

ntuple_impl! {T0,}

//ntuple_impl! {T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, }
