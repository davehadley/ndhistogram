use super::axis::Axis;

/// Axes provided an interface for a set of ND dimensional set of histograms.
pub trait Axes: Axis {
    /// Returns the number of axes within this set.
    fn num_dim(&self) -> usize;
}

// Count idents in macro from: <https://danielkeep.github.io/tlborm/book/blk-counting.html>
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

macro_rules! impl_axes {
    () => {
    };
    //( ($index:tt => $type_parameter:ident), ) => {
        ( $type_parameter:ident: $index:tt, ) => {

        impl<X: Axis> Axes for (X,) {
            fn num_dim(&self) -> usize { 1 }
        }

        impl<X: Axis> Axis for (X,) {
            type Coordinate = X::Coordinate;
            type BinInterval = X::BinInterval;

            fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
                self.0.index(coordinate)
            }

            fn num_bins(&self) -> usize {
                self.0.num_bins()
            }

            fn bin(&self, index: usize) -> Option<Self::BinInterval> {
                self.0.bin(index)
            }
        }

        impl_axes!();
    };
    //( $( ($nth_index:tt => $nth_type_parameter:ident), )+ ) => {
        ( $($nth_type_parameter:ident: $nth_index:tt, )+ ) => {
        impl<$($nth_type_parameter: Axis),*> Axes for ($($nth_type_parameter),*) {
            fn num_dim(&self) -> usize { count_idents!($($nth_type_parameter),*) }
        }

        impl<$($nth_type_parameter: Axis),*> Axis for ($($nth_type_parameter),*) {
            type Coordinate = ($($nth_type_parameter::Coordinate),*);
            type BinInterval = ($($nth_type_parameter::BinInterval),*);

            fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
                let num_bins: Vec<_> = [$(self.$nth_index.num_bins()),*].iter().scan(1, |acc, nbin| {*acc *= *nbin; Some(*acc)}).collect();
                let indices = [$(self.$nth_index.index(&coordinate.$nth_index)?),*];

                let index = num_bins.iter()
                    .rev()
                    .skip(1)
                    .zip(indices.iter().rev())
                    .fold(indices[0], |acc, (nbin, idx)| acc + nbin*idx);
                Some(index)
            }

            fn num_bins(&self) -> usize {
                //let arr = [self.$index.num_bins(), $(self.$nth_index.num_bins()),*];
                $(self.$nth_index.num_bins()*)* 1
            }

            fn bin(&self, index: usize) -> Option<Self::BinInterval> {
                let num_bins = [$(self.$nth_index.num_bins()),*];
                let product = num_bins.iter().scan(1, |acc, it| Some(*acc * *it));
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

#[derive(Default)]
pub struct AxesAnyD<C, I> {
    axes: Vec<Box<dyn Axis<Coordinate = C, BinInterval = I>>>,
    shape: Vec<usize>,
    cumulative_shape_product: Vec<usize>,
    num_bins: usize,
}

impl<C, I> AxesAnyD<C, I> {
    pub fn new<
        'a,
        Iterator: IntoIterator<Item = Box<dyn Axis<Coordinate = C, BinInterval = I>>> + 'a,
    >(
        axes: Iterator,
    ) -> Self {
        let axes: Vec<_> = axes.into_iter().collect();
        let shape: Vec<_> = axes.iter().map(|it| it.num_bins()).collect();
        let num_bins = shape.iter().fold(1, |left, right| left * right);
        let cumulative_shape_product: Vec<_> = shape
            .iter()
            .scan(1, |acc, nbin| {
                *acc *= *nbin;
                Some(*acc)
            })
            .collect();
        Self {
            axes,
            shape,
            cumulative_shape_product,
            num_bins,
        }
    }
}

impl<C, I> Axes for AxesAnyD<C, I> {
    fn num_dim(&self) -> usize {
        self.axes.len()
    }
}

impl<C, I> Axis for AxesAnyD<C, I> {
    type Coordinate = Vec<C>;

    type BinInterval = Vec<I>;

    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
        let indices: Vec<_> = self
            .axes
            .iter()
            .zip(coordinate)
            .map(|(ax, c)| ax.index(c))
            .flatten()
            .collect();
        if indices.len() != coordinate.len() || indices.len() != self.axes.len() {
            return None;
        }
        let index = self
            .shape
            .iter()
            .rev()
            .skip(1)
            .zip(indices.iter().rev())
            .fold(indices[0], |acc, (nbin, idx)| acc + nbin * idx);
        Some(index)
    }

    fn num_bins(&self) -> usize {
        self.num_bins
    }

    fn bin(&self, index: usize) -> Option<Self::BinInterval> {
        let mut index = index;
        let index: Vec<_> = self
            .cumulative_shape_product
            .iter()
            .map(|nb| {
                let v = index % nb;
                index /= nb;
                v
            })
            .collect();
        self.axes
            .iter()
            .zip(index)
            .map(|(ax, ind)| ax.bin(ind))
            .collect()
    }
}
