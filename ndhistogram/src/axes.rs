use ndarray::Array3;

use crate::histogram::Grow;

use super::axis::Axis;

// impl<X: Axis, Y: Axis> Axis for (X, Y) {
//     type Coordinate = (X::Coordinate, Y::Coordinate);
//     type BinRange = (X::BinRange, Y::BinRange);

//     fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
//         let ix = self.0.index(&coordinate.0)?;
//         let iy = self.1.index(&coordinate.1)?;
//         Some(ix + self.0.numbins() * iy)
//     }

//     fn numbins(&self) -> usize {
//         self.0.numbins() * self.1.numbins()
//     }

//     fn bin(&self, index: usize) -> Option<Self::BinRange> {
//         let ix = index % self.0.numbins();
//         let iy = index / self.0.numbins();

//         let bx = self.0.bin(ix)?;
//         let by = self.1.bin(iy)?;
//         Some((bx, by))
//     }
// }

macro_rules! impl_axes {
    () => {
        pub trait Axes: Axis {}
    };
    //( ($index:tt => $type_parameter:ident), ) => {
        ( $type_parameter:ident: $index:tt, ) => {

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

        impl_axes!();
    };
    //( $( ($nth_index:tt => $nth_type_parameter:ident), )+ ) => {
        ( $($nth_type_parameter:ident: $nth_index:tt, )+ ) => {
        impl<$($nth_type_parameter: Axis),*> Axes for ($($nth_type_parameter),*) {}

        impl<$($nth_type_parameter: Axis),*> Axis for ($($nth_type_parameter),*) {
            type Coordinate = ($($nth_type_parameter::Coordinate),*);
            type BinRange = ($($nth_type_parameter::BinRange),*);

            fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
                let numbins: Vec<_> = [$(self.$nth_index.numbins()),*].iter().scan(1, |acc, nbin| {*acc *= *nbin; Some(*acc)}).collect();
                let indices = [$(self.$nth_index.index(&coordinate.$nth_index)?),*];

                let index = numbins.iter()
                    .rev()
                    .skip(1)
                    .zip(indices.iter().rev())
                    .fold(indices[0], |acc, (nbin, idx)| acc + nbin*idx);
                Some(index)
            }

            fn numbins(&self) -> usize {
                //let arr = [self.$index.numbins(), $(self.$nth_index.numbins()),*];
                $(self.$nth_index.numbins()*)* 1
            }

            fn bin(&self, index: usize) -> Option<Self::BinRange> {
                let numbins = [$(self.$nth_index.numbins()),*];
                let product = numbins.iter().scan(1, |acc, it| Some(*acc * *it));
                let mut index = index;
                let index: Vec<_> = product.map(|nb| {
                    let v = index % nb;
                    index /= nb;
                    v
                } ).collect();
                Some(
                    (
                        $(self.$nth_index.bin(index[$nth_index])?),*
                )
            )
            }
        }

        impl_axes!(@REMOVELAST $(($nth_index => $nth_type_parameter),)*);
    };
    (@REMOVELAST ($index:tt => $type_parameter:ident), $( ($nth_index:tt => $nth_type_parameter:ident), )+ ) => {
        impl_axes!(@REMOVELAST ($index => $type_parameter), @SEPARATOR $(($nth_index => $nth_type_parameter),)*);
    };
    (@REMOVELAST $( ($first_index:tt => $first_type_parameter:ident), )+ @SEPARATOR ($index:tt => $type_parameter:ident), $( ($nth_index:tt => $nth_type_parameter:ident), )+ ) => {
        impl_axes!(@REMOVELAST $(($first_index => $first_type_parameter),)* ($index => $type_parameter), @SEPARATOR $(($nth_index => $nth_type_parameter),)*);
    };
    (@REMOVELAST $( ($first_index:tt => $first_type_parameter:ident), )+ @SEPARATOR ($index:tt => $type_parameter:ident), ) => {
        //impl_axes!($(($first_index => $first_type_parameter),)*);
        impl_axes!($($first_type_parameter: $first_index,)*);
    };
}

// impl_axes! {
//     (0 => X),
//     (1 => Y),
//     (2 => Z),
//     (3 => T),
//     (4 => T1),
//     (5 => T2),
//     (6 => T3),
//     (7 => T4),
//     (8 => T5),
//     (9 => T6),
//     (10 => T7),
//     (11 => T8),
//     (12 => T9),
//     (13 => T10),
// }
impl_axes! {
    x: 0,
    y: 1,
    z: 2,
    t: 3,
    d4: 4,
    d5: 5,
    d6: 6,
    d7: 7,
    d8: 8,
    d9: 9,
    d10: 10,
    d11: 11,
    d12: 12,
    d13: 13,
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

fn blha() {
    use ndarray::Array3;
    let mut temperature = Array3::<f64>::zeros((3, 4, 5));
    // Increase the temperature in this location
    temperature[[2, 2, 2]] += 0.5;
}
