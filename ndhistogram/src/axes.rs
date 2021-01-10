use ndarray::Array3;

use super::axis::Axis;

macro_rules! impl_axes {
    () => {

        pub trait Axes : Axis {

        }

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
        pub struct AxesTuple<T> { axes: T }

        impl <T> core::ops::Deref for AxesTuple<T> {
            type Target = T;

            fn deref (self: &'_ Self) -> &'_ Self::Target
            {
                &self.axes
            }
        }

    };
    //( ($index:tt => $type_parameter:ident), ) => {
        ( $type_parameter:ident: $index:tt, ) => {

        impl<X: Axis> AxesTuple<(X,)> {
            pub fn new(axes: (X,)) -> AxesTuple<(X,)> { AxesTuple{axes} }
        }

        impl<X: Axis> From<(X,)> for AxesTuple<(X,)> {
            fn from(item: (X,)) -> Self { Self::new(item) }
        }

        impl<X: Axis> Axes for AxesTuple<(X,)> {}

        impl<X: Axis> Axis for AxesTuple<(X,)> {
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
        impl<$($nth_type_parameter: Axis),*> AxesTuple<($($nth_type_parameter),*)> {}

        impl<$($nth_type_parameter: Axis),*> Axes for AxesTuple<($($nth_type_parameter),*)> {}

        impl<$($nth_type_parameter: Axis),*> AxesTuple<($($nth_type_parameter),*)> {
            pub fn new(axes: ($($nth_type_parameter),*)) -> AxesTuple<($($nth_type_parameter),*)> { AxesTuple{axes} }
        }

        impl<$($nth_type_parameter: Axis),*> From<($($nth_type_parameter),*)> for AxesTuple<($($nth_type_parameter),*)> {
            fn from(item: ($($nth_type_parameter),*)) -> Self { Self::new(item) }
        }

        impl<$($nth_type_parameter: Axis),*> Axis for AxesTuple<($($nth_type_parameter),*)> {
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

        impl_axes!(@REMOVELAST $([$nth_index AND $nth_type_parameter],)*);
    };

    // TODO: yuk! so ugly. clean this up by moving it out into another macro
    // Remove the last element of the impl_axes! { x: 0 .... n-1: N-1, n: N}
    // and call impl_axes! { x: 0 .... n-1: N-1}
    (@REMOVELAST [$index:tt AND $type_parameter:ident], $( [$nth_index:tt AND $nth_type_parameter:ident], )+ ) => {
        impl_axes!(@REMOVELAST [$index AND $type_parameter], @SEPARATOR $([$nth_index AND $nth_type_parameter],)*);
    };
    // optimisation to reduce levels of recursion required (move 4 at once)
    (@REMOVELAST $( [$first_index:tt AND $first_type_parameter:ident], )+ @SEPARATOR [$index1:tt AND $type_parameter1:ident], [$index2:tt AND $type_parameter2:ident], [$index3:tt AND $type_parameter3:ident], [$index4:tt AND $type_parameter4:ident], $( [$nth_index:tt AND $nth_type_parameter:ident], )+ ) => {
        impl_axes!(@REMOVELAST $([$first_index AND $first_type_parameter],)* [$index1 AND $type_parameter1], [$index2 AND $type_parameter2], [$index3 AND $type_parameter3], [$index4 AND $type_parameter4], @SEPARATOR $([$nth_index AND $nth_type_parameter],)*);
    };
    // optimisation to reduce levels of recursion required (move 2 at once)
    (@REMOVELAST $( [$first_index:tt AND $first_type_parameter:ident], )+ @SEPARATOR [$index1:tt AND $type_parameter1:ident], [$index2:tt AND $type_parameter2:ident], $( [$nth_index:tt AND $nth_type_parameter:ident], )+ ) => {
        impl_axes!(@REMOVELAST $([$first_index AND $first_type_parameter],)* [$index1 AND $type_parameter1], [$index2 AND $type_parameter2], @SEPARATOR $([$nth_index AND $nth_type_parameter],)*);
    };
    (@REMOVELAST $( [$first_index:tt AND $first_type_parameter:ident], )+ @SEPARATOR [$index:tt AND $type_parameter:ident], $( [$nth_index:tt AND $nth_type_parameter:ident], )+ ) => {
        impl_axes!(@REMOVELAST $([$first_index AND $first_type_parameter],)* [$index AND $type_parameter], @SEPARATOR $([$nth_index AND $nth_type_parameter],)*);
    };
    // all pairs have been moved to the left
    (@REMOVELAST $( [$first_index:tt AND $first_type_parameter:ident], )+ @SEPARATOR [$index:tt AND $type_parameter:ident], ) => {
        //impl_axes!($(($first_index => $first_type_parameter),)*);
        impl_axes!($($first_type_parameter: $first_index,)*);
    };
}

impl_axes! {
    X: 0,
    Y: 1,
    Z: 2,
    T: 3,
    D4: 4,
    D5: 5,
    D6: 6,
    D7: 7,
    D8: 8,
    D9: 9,
    D10: 10,
    D11: 11,
    D12: 12,
    D13: 13,
    D14: 14,
    D15: 15,
    D16: 16,
    D17: 17,
    D18: 18,
    D19: 19,
    D20: 20,
}
