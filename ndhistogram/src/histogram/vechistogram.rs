use std::{
    cmp::Ordering,
    f64::INFINITY,
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::{axis::Axis, error::BinaryOperationError};

use super::histogram::{Histogram, Item, Iter, IterMut, ValuesMut};

/// A [Histogram] that stores its values in a [Vec].
///
/// See [ndhistogram] for examples of its use.
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecHistogram<A, V> {
    axes: A,
    values: Vec<V>,
}

impl<A: Axis, V: Default + Clone> VecHistogram<A, V> {
    /// Factor method for VecHistogram. It is recommended to use the
    /// [ndhistogram](crate::ndhistogram) macro instead.
    pub fn new(axes: A) -> Self {
        let size = axes.num_bins();
        Self {
            axes,
            values: vec![V::default(); size],
        }
    }
}

impl<A: Axis, V> Histogram<A, V> for VecHistogram<A, V> {
    fn value(&self, coordinate: &A::Coordinate) -> Option<&V> {
        let index = self.axes.index(coordinate)?;
        self.values.get(index)
    }

    #[inline]
    fn axes(&self) -> &A {
        &self.axes
    }

    fn value_at_index(&self, index: usize) -> Option<&V> {
        self.values.get(index)
    }

    fn values<'a>(&'a self) -> Box<dyn Iterator<Item = &'a V> + 'a> {
        Box::new(self.values.iter())
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = Item<A::BinInterval, &'a V>> + 'a> {
        Box::new(self.axes().iter().map(move |(index, binrange)| {
            Item {
                index,
                bin: binrange,
                value: self
                    .value_at_index(index)
                    .expect("iter() indices are always in range"),
            }
        }))
    }

    fn value_at_index_mut(&mut self, index: usize) -> Option<&mut V> {
        self.values.get_mut(index)
    }

    fn values_mut(&mut self) -> ValuesMut<'_, V> {
        Box::new(self.values.iter_mut())
    }

    fn iter_mut(&mut self) -> IterMut<'_, A, V> {
        Box::new(
            self.axes
                .iter()
                .zip(self.values.iter_mut())
                .map(|((index, bin), value)| Item { index, bin, value }),
        )
    }
}

impl<'a, A: Axis, V> IntoIterator for &'a VecHistogram<A, V> {
    type Item = Item<A::BinInterval, &'a V>;

    type IntoIter = Iter<'a, A, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, A: Axis, V: 'a> IntoIterator for &'a mut VecHistogram<A, V> {
    type Item = Item<A::BinInterval, &'a mut V>;

    type IntoIter = IterMut<'a, A, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<A: Axis, V> Display for VecHistogram<A, V>
where
    V: Clone + Into<f64>,
    A::BinInterval: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(2);

        let sum = self
            .values()
            .map(|it| {
                let x: f64 = it.clone().into();
                x
            })
            .fold(0.0, |it, value| it + value);
        write!(
            f,
            "VecHistogram{}D({} bins, sum={})",
            self.axes().num_dim(),
            self.axes().num_bins(),
            sum
        )?;
        let values: Vec<_> = self
            .iter()
            .take(50)
            .map(|item| {
                (item.bin, {
                    let x: f64 = item.value.clone().into();
                    x
                })
            })
            .collect();
        let scale = values
            .iter()
            .max_by(|l, r| l.1.partial_cmp(&r.1).unwrap_or(Ordering::Less))
            .map(|it| it.1)
            .unwrap_or(INFINITY);
        values
            .into_iter()
            .map(|(bin, value)| (bin, 50.0 * (value / scale)))
            .map(|(bin, value)| {
                (
                    format!("{:.precision$}", bin, precision = precision),
                    "#".repeat(value as usize),
                )
            })
            .map(|(bin, value)| write!(f, "\n{:>16} | {}", bin, value))
            .filter_map(Result::ok)
            .count();
        Ok(())
    }
}

macro_rules! impl_binary_op_with_immutable_borrow {
    ($Trait:tt, $method:tt, $mathsymbol:tt, $testresult:tt) => {
        impl<A: Axis + PartialEq + Clone, V> $Trait<&VecHistogram<A, V>> for &VecHistogram<A, V>
where
    for<'a> &'a V: $Trait<Output = V>,
{
    type Output = Result<VecHistogram<A, V>, BinaryOperationError>;

    /// Combine the right-hand histogram with the left-hand histogram,
    /// returning a copy, and leaving the original histograms intact.
    ///
    /// If the input histograms have incompatible axes, this operation
    /// will return a [BinaryOperationError].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ndhistogram::{Histogram, ndhistogram, axis::Uniform};
    /// let mut hist1 = ndhistogram!(Uniform::<f64>::new(10, -5.0, 5.0));
    /// let mut hist2 = ndhistogram!(Uniform::<f64>::new(10, -5.0, 5.0));
    /// hist1.fill_with(&0.0, 2.0);
    /// hist2.fill(&0.0);
    #[doc=concat!("let combined_hist = (&hist1 ", stringify!($mathsymbol), " &hist2).expect(\"Axes are compatible\");")]
    #[doc=concat!("assert_eq!(combined_hist.value(&0.0).unwrap(), &", stringify!($testresult), ");")]
    fn $method(self, rhs: &VecHistogram<A, V>) -> Self::Output {
        if self.axes() != rhs.axes() {
            return Err(BinaryOperationError);
        }
        let values = self
            .values
            .iter()
            .zip(rhs.values.iter())
            .map(|(l, r)| l $mathsymbol r)
            .collect();
        Ok(VecHistogram {
            axes: self.axes().clone(),
            values,
        })
    }
}
    };
}

impl_binary_op_with_immutable_borrow! {Add, add, +, 3.0}
impl_binary_op_with_immutable_borrow! {Sub, sub, -, 1.0}
impl_binary_op_with_immutable_borrow! {Mul, mul, *, 2.0}
impl_binary_op_with_immutable_borrow! {Div, div, /, 2.0}

macro_rules! impl_binary_op_with_scalar {
    ($Trait:tt, $method:tt, $mathsymbol:tt) => {
        impl<A: Axis + PartialEq + Clone, V> $Trait<&V> for &VecHistogram<A, V>
where
    for<'a> &'a V: $Trait<Output = V>,
{
    type Output = VecHistogram<A, V>;

    fn $method(self, rhs: &V) -> Self::Output {
        let values = self
            .values
            .iter()
            .map(|l| l $mathsymbol rhs)
            .collect();
        VecHistogram {
            axes: self.axes().clone(),
            values,
        }
    }
}
    };
}

impl_binary_op_with_scalar! {Add, add, +}
impl_binary_op_with_scalar! {Sub, sub, -}
impl_binary_op_with_scalar! {Mul, mul, *}
impl_binary_op_with_scalar! {Div, div, /}

macro_rules! impl_binary_op_with_owned {
    ($Trait:tt, $method:tt, $ValueAssignTrait:tt, $mathsymbol:tt, $assignmathsymbol:tt, $testresult:tt) => {
        impl<A: Axis + PartialEq, V> $Trait<&VecHistogram<A, V>> for VecHistogram<A, V>
        where
            for<'a> V: $ValueAssignTrait<&'a V>,
        {
            type Output = Result<VecHistogram<A, V>, BinaryOperationError>;

            /// Combine the right-hand histogram with the left-hand histogram,
            /// consuming the left-hand histogram and returning a new value.
            /// As this avoids making copies of the histograms, this is the
            /// recommended method to merge histograms.
            ///
            /// If the input histograms have incompatible axes, this operation
            /// will return a [BinaryOperationError].
            ///
            /// # Examples
            ///
            /// ```rust
            /// use ndhistogram::{Histogram, ndhistogram, axis::Uniform};
            /// let mut hist1 = ndhistogram!(Uniform::<f64>::new(10, -5.0, 5.0));
            /// let mut hist2 = ndhistogram!(Uniform::<f64>::new(10, -5.0, 5.0));
            /// hist1.fill_with(&0.0, 2.0);
            /// hist2.fill(&0.0);
            #[doc=concat!("let combined_hist = (hist1 ", stringify!($mathsymbol), " &hist2).expect(\"Axes are compatible\");")]
            #[doc=concat!("assert_eq!(combined_hist.value(&0.0).unwrap(), &", stringify!($testresult), ");")]
            fn $method(mut self, rhs: &VecHistogram<A, V>) -> Self::Output {
                if self.axes() != rhs.axes() {
                    return Err(BinaryOperationError);
                }
                self.values
                    .iter_mut()
                    .zip(rhs.values.iter())
                    .for_each(|(l, r)| *l $assignmathsymbol &r);
                Ok(self)
            }
        }
    };
}

impl_binary_op_with_owned! {Add, add, AddAssign, +, +=, 3.0}
impl_binary_op_with_owned! {Sub, sub, SubAssign, -, -=, 1.0}
impl_binary_op_with_owned! {Mul, mul, MulAssign, *, *=, 2.0}
impl_binary_op_with_owned! {Div, div, DivAssign, /, /=, 2.0}

macro_rules! impl_binary_op_assign {
    ($Trait:tt, $method:tt, $ValueAssignTrait:tt, $mathsymbol:tt, $testresult:tt) => {
        impl<A: Axis + PartialEq, V> $Trait<&VecHistogram<A, V>> for VecHistogram<A, V>
        where
            for<'a> V: $ValueAssignTrait<&'a V>,
        {
            /// Combine the right-hand histogram with the left-hand histogram,
            /// mutating the left-hand histogram.
            ///
            /// # Panics
            ///
            /// Panics if the histograms have incompatible axes.
            /// To handle this failure mode at runtime, use the non-assign
            /// version of this operation, which returns an Result.
            ///
            /// # Examples
            ///
            /// ```rust
            /// use ndhistogram::{Histogram, ndhistogram, axis::Uniform};
            /// let mut hist1 = ndhistogram!(Uniform::<f64>::new(10, -5.0, 5.0));
            /// let mut hist2 = ndhistogram!(Uniform::<f64>::new(10, -5.0, 5.0));
            /// hist1.fill_with(&0.0, 2.0);
            /// hist2.fill(&0.0);
            #[doc=concat!("hist1 ", stringify!($mathsymbol), " &hist2;")]
            #[doc=concat!("assert_eq!(hist1.value(&0.0).unwrap(), &", stringify!($testresult), ");")]
            /// ```
            fn $method(&mut self, rhs: &VecHistogram<A, V>) {
                if self.axes() != rhs.axes() {
                    panic!("Cannot combine VecHistograms with incompatible axes.");
                }
                self.values
                    .iter_mut()
                    .zip(rhs.values.iter())
                    .for_each(|(l, r)| *l $mathsymbol &r);
            }
        }
    };
}

impl_binary_op_assign! {AddAssign, add_assign, AddAssign, +=, 3.0}
impl_binary_op_assign! {SubAssign, sub_assign, SubAssign, -=, 1.0}
impl_binary_op_assign! {MulAssign, mul_assign, MulAssign, *=, 2.0}
impl_binary_op_assign! {DivAssign, div_assign, DivAssign, /=, 2.0}
