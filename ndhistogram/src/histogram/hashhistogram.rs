use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    fmt::Display,
    hash::{BuildHasher, Hasher},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use super::histogram::{Histogram, Iter, IterMut, ValuesMut};
use crate::{axis::Axis, error::AxisError, Axes, Item};

use rustc_hash::FxHasher;

#[derive(Default, Clone)]
/// Default hashing algorithm used for HashHistogram.
///
/// Currently uses rustc-hash FxHasher.
pub struct DefaultHasher(FxHasher);

impl BuildHasher for DefaultHasher {
    type Hasher = Self;

    fn build_hasher(&self) -> Self::Hasher {
        Self::default()
    }
}

impl Hasher for DefaultHasher {
    fn finish(&self) -> u64 {
        self.0.finish()
    }
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }
}

impl Debug for DefaultHasher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DefaultHasher").finish()
    }
}

/// A sparse N-dimensional [Histogram] that stores its values in a [HashMap].
///
/// Only bins that are filled will consume memory.
/// This makes high-dimensional, many-binned (but mostly empty) histograms
///  possible. If memory usage is not a concern, see [VecHistogram](crate::VecHistogram).
///
/// See [crate::sparsehistogram] for examples of its use.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        deserialize = "A: serde::Deserialize<'de>, V: serde::Deserialize<'de>, S: std::hash::BuildHasher + Default"
    ))
)]
pub struct HashHistogram<A, V, S = DefaultHasher> {
    axes: A,
    values: HashMap<usize, V, S>,
}

impl<A, V, S> Eq for HashHistogram<A, V, S>
where
    A: Eq,
    V: Eq,
    S: std::hash::BuildHasher,
{
}

impl<A, V, S> PartialEq for HashHistogram<A, V, S>
where
    A: PartialEq,
    V: PartialEq,
    S: std::hash::BuildHasher,
{
    fn eq(&self, other: &Self) -> bool {
        self.axes == other.axes && self.values == other.values
    }
}

impl<A: Axis, V> HashHistogram<A, V> {
    /// Factory method for HashHistogram. It is recommended to use the
    /// [sparsehistogram](crate::sparsehistogram) macro instead.
    pub fn new(axes: A) -> Self {
        Self {
            axes,
            values: HashMap::default(),
        }
    }
}

impl<A: Axis, V, S: std::hash::BuildHasher> HashHistogram<A, V, S> {
    /// Factory method for HashHistogram to specify the hashing method.
    pub fn with_hasher(axes: A, hash_builder: S) -> Self {
        Self {
            axes,
            values: HashMap::with_hasher(hash_builder),
        }
    }
}

impl<A: Axis, V: Default, S: BuildHasher> Histogram<A, V> for HashHistogram<A, V, S> {
    #[inline]
    fn axes(&self) -> &A {
        &self.axes
    }

    fn value_at_index(&self, index: usize) -> Option<&V> {
        self.values.get(&index)
    }

    fn values(&self) -> super::histogram::Values<'_, V> {
        Box::new(self.values.values())
    }

    fn iter(&self) -> Iter<'_, A, V> {
        Box::new(self.values.iter().map(move |(index, value)| {
            Item {
                index: *index,
                bin: self
                    .axes
                    .bin(*index)
                    .expect("iter() indices are always valid bins"),
                value,
            }
        }))
    }

    fn value_at_index_mut(&mut self, index: usize) -> Option<&mut V> {
        self.values.get_mut(&index)
    }

    fn values_mut(&mut self) -> ValuesMut<'_, V> {
        Box::new(self.values.values_mut())
    }

    fn iter_mut(&mut self) -> IterMut<'_, A, V> {
        let axes = &self.axes;
        Box::new(self.values.iter_mut().map(move |(index, value)| {
            Item {
                index: *index,
                bin: axes
                    .bin(*index)
                    .expect("iter_mut() indices are always valid bins"),
                value,
            }
        }))
    }

    #[inline]
    fn fill(&mut self, coordinate: &A::Coordinate)
    where
        V: crate::Fill,
    {
        if let Some(index) = self.axes.index(coordinate) {
            self.values.entry(index).or_default().fill();
        }
    }

    #[inline]
    fn fill_with<D>(&mut self, coordinate: &A::Coordinate, data: D)
    where
        V: crate::FillWith<D>,
    {
        if let Some(index) = self.axes.index(coordinate) {
            self.values.entry(index).or_default().fill_with(data);
        }
    }

    #[inline]
    fn fill_with_weighted<D, W>(&mut self, coordinate: &A::Coordinate, data: D, weight: W)
    where
        V: crate::FillWithWeighted<D, W>,
    {
        if let Some(index) = self.axes.index(coordinate) {
            self.values
                .entry(index)
                .or_default()
                .fill_with_weighted(data, weight);
        }
    }
}

impl<'a, A: Axis, V, S> IntoIterator for &'a HashHistogram<A, V, S>
where
    HashHistogram<A, V, S>: Histogram<A, V>,
{
    type Item = Item<A::BinInterval, &'a V>;

    type IntoIter = Iter<'a, A, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, A: Axis, V: 'a, S> IntoIterator for &'a mut HashHistogram<A, V, S>
where
    HashHistogram<A, V, S>: Histogram<A, V>,
{
    type Item = Item<A::BinInterval, &'a mut V>;

    type IntoIter = IterMut<'a, A, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<A: Axis, V, S> Display for HashHistogram<A, V, S>
where
    HashHistogram<A, V, S>: Histogram<A, V>,
    V: Clone + Into<f64>,
    A::BinInterval: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sum = self
            .values()
            .map(|it| {
                let x: f64 = it.clone().into();
                x
            })
            .fold(0.0, |it, value| it + value);
        write!(
            f,
            "HashHistogram{}D({} bins, sum={})",
            self.axes().num_dim(),
            self.axes().num_bins(),
            sum
        )?;
        Ok(())
    }
}

macro_rules! impl_binary_op_with_immutable_borrow {
    ($Trait:tt, $method:tt, $mathsymbol:tt, $testresult:tt) => {
        impl<A: Axis + PartialEq + Clone, V, S> $Trait<&HashHistogram<A, V, S>> for &HashHistogram<A, V, S>
        where
            HashHistogram<A, V, S>: Histogram<A, V>,
            V: Clone + Default,
            S: BuildHasher + Default,
            for<'a> &'a V: $Trait<Output = V>,
        {
            type Output = Result<HashHistogram<A, V, S>, crate::error::BinaryOperationError>;

            /// Combine the right-hand histogram with the left-hand histogram,
            /// returning a copy, and leaving the original histograms intact.
            ///
            /// If the input histograms have incompatible axes, this operation
            /// will return a [crate::error::BinaryOperationError].
            ///
            /// # Examples
            ///
            /// ```rust
            /// use ndhistogram::{Histogram, sparsehistogram, axis::Uniform};
            /// # fn main() -> Result<(), ndhistogram::Error> {
            /// let mut hist1 = sparsehistogram!(Uniform::<f64>::new(10, -5.0, 5.0)?);
            /// let mut hist2 = sparsehistogram!(Uniform::<f64>::new(10, -5.0, 5.0)?);
            /// hist1.fill_with(&0.0, 2.0);
            /// hist2.fill(&0.0);
            #[doc=concat!("let combined_hist = (&hist1 ", stringify!($mathsymbol), " &hist2).expect(\"Axes are compatible\");")]
            #[doc=concat!("assert_eq!(combined_hist.value(&0.0).unwrap(), &", stringify!($testresult), ");")]
            /// # Ok(()) }
            /// ```
            fn $method(self, rhs: &HashHistogram<A, V, S>) -> Self::Output {
                if self.axes() != rhs.axes() {
                    return Err(crate::error::BinaryOperationError);
                }
                let indices: HashSet<usize> = self.values.keys().chain(rhs.values.keys()).copied().collect();
                let values: HashMap<usize, V, S> = indices.into_iter().map(|index| {
                    let left = self.values.get(&index);
                    let right = rhs.values.get(&index);
                    let newvalue = match (left, right) {
                        (Some(left), Some(right)) => { left $mathsymbol right },
                        (None, Some(right)) => { &V::default() $mathsymbol right },
                        (Some(left), None) => { left $mathsymbol &V::default() },
                        (None, None) => { unreachable!() },
                    };
                    (index, newvalue)
                }).collect();
                Ok(HashHistogram {
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

macro_rules! impl_binary_op_with_owned {
    ($Trait:tt, $method:tt, $ValueAssignTrait:tt, $mathsymbol:tt, $assignmathsymbol:tt, $testresult:tt) => {
        impl<A: Axis + PartialEq + Clone, V, S> $Trait<&HashHistogram<A, V, S>> for HashHistogram<A, V, S>
        where
            HashHistogram<A, V, S>: Histogram<A, V>,
            V: Clone + Default,
            S: BuildHasher,
            for<'a> V: $ValueAssignTrait<&'a V>,
        {
            type Output = Result<HashHistogram<A, V, S>, crate::error::BinaryOperationError>;

            /// Combine the right-hand histogram with the left-hand histogram,
            /// consuming the left-hand histogram and returning a new value.
            /// As this avoids making copies of the histograms, this is the
            /// recommended method to merge histograms.
            ///
            /// If the input histograms have incompatible axes, this operation
            /// will return a [crate::error::BinaryOperationError].
            ///
            /// # Examples
            ///
            /// ```rust
            /// use ndhistogram::{Histogram, sparsehistogram, axis::Uniform};
            /// # fn main() -> Result<(), ndhistogram::Error> {
            /// let mut hist1 = sparsehistogram!(Uniform::<f64>::new(10, -5.0, 5.0)?);
            /// let mut hist2 = sparsehistogram!(Uniform::<f64>::new(10, -5.0, 5.0)?);
            /// hist1.fill_with(&0.0, 2.0);
            /// hist2.fill(&0.0);
            #[doc=concat!("let combined_hist = (hist1 ", stringify!($mathsymbol), " &hist2).expect(\"Axes are compatible\");")]
            #[doc=concat!("assert_eq!(combined_hist.value(&0.0).unwrap(), &", stringify!($testresult), ");")]
            /// # Ok(()) }
            /// ```
            fn $method(mut self, rhs: &HashHistogram<A, V, S>) -> Self::Output {
                if self.axes() != rhs.axes() {
                    return Err(crate::error::BinaryOperationError);
                }
                for (index, rhs_value) in rhs.values.iter() {
                    let lhs_value = self.values.entry(*index).or_default();
                    *lhs_value $assignmathsymbol rhs_value
                }
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
        impl<A: Axis + PartialEq, V, S> $Trait<&HashHistogram<A, V, S>> for HashHistogram<A, V, S>
        where
            HashHistogram<A, V, S>: Histogram<A, V>,
            V: Default,
            S: BuildHasher,
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
            /// use ndhistogram::{Histogram, sparsehistogram, axis::Uniform};
            /// # fn main() -> Result<(), ndhistogram::Error> {
            /// let mut hist1 = sparsehistogram!(Uniform::<f64>::new(10, -5.0, 5.0)?);
            /// let mut hist2 = sparsehistogram!(Uniform::<f64>::new(10, -5.0, 5.0)?);
            /// hist1.fill_with(&0.0, 2.0);
            /// hist2.fill(&0.0);
            #[doc=concat!("hist1 ", stringify!($mathsymbol), " &hist2;")]
            #[doc=concat!("assert_eq!(hist1.value(&0.0).unwrap(), &", stringify!($testresult), ");")]
            /// # Ok(()) }
            /// ```
            fn $method(&mut self, rhs: &HashHistogram<A, V, S>) {
                if self.axes() != rhs.axes() {
                    panic!("Cannot combine HashHistograms with incompatible axes.");
                }
                for (index, rhs_value) in rhs.values.iter() {
                    let lhs_value = self.values.entry(*index).or_default();
                    *lhs_value $mathsymbol rhs_value
                }
            }
        }
    };
}

impl_binary_op_assign! {AddAssign, add_assign, AddAssign, +=, 3.0}
impl_binary_op_assign! {SubAssign, sub_assign, SubAssign, -=, 1.0}
impl_binary_op_assign! {MulAssign, mul_assign, MulAssign, *=, 2.0}
impl_binary_op_assign! {DivAssign, div_assign, DivAssign, /=, 2.0}

#[cfg(feature = "rayon")]
use rayon::prelude::*;

// TODO: It would be better to implement rayon::iter::IntoParallelIterator
// See comments on vechistogram for more info.

impl<A, V, S> HashHistogram<A, V, S> {
    /// Construct a HashHistogram from a Vec and Axes.
    ///
    /// Returns AxisError::InvalidNumberOfBins if the provided values contains bins indices that are not contained within the provided axes.
    pub fn from_map(axes: A, values: HashMap<usize, V, S>) -> Result<Self, crate::Error>
    where
        A: Axes,
    {
        if values.keys().all(|it| *it < axes.num_bins()) {
            Ok(Self { axes, values })
        } else {
            Err(AxisError::InvalidNumberOfBins.into())
        }
    }

    /// Get a reference to the backing HashMap.
    pub fn as_map(&self) -> &HashMap<usize, V, S> {
        &self.values
    }

    /// An [immutable rayon parallel iterator](rayon::iter::ParallelIterator) over the histogram values.
    ///
    /// It only iterates over filled bins in the sparse histogram.
    /// This requires the "rayon" [crate feature](index.html#crate-feature-flags) to be enabled.
    #[cfg(feature = "rayon")]
    pub fn par_values(&self) -> impl ParallelIterator<Item = &V>
    where
        V: Sync,
    {
        self.values.par_iter().map(|it| it.1)
    }

    /// A [mutable rayon parallel iterator](rayon::iter::ParallelIterator) over the histogram values.
    ///
    /// It only iterates over filled bins in the sparse histogram.
    /// This requires the "rayon" [crate feature](index.html#crate-feature-flags) to be enabled.
    #[cfg(feature = "rayon")]
    pub fn par_values_mut(&mut self) -> impl ParallelIterator<Item = &mut V>
    where
        V: Send,
    {
        self.values.par_iter_mut().map(|it| it.1)
    }

    /// An [immutable rayon parallel iterator](rayon::iter::ParallelIterator) over bin indices, bin interval and bin values.
    ///
    /// It only iterates over filled bins in the sparse histogram.
    /// This requires the "rayon" [crate feature](index.html#crate-feature-flags) to be enabled.
    #[cfg(feature = "rayon")]
    pub fn par_iter(&self) -> impl ParallelIterator<Item = Item<<A as Axis>::BinInterval, &V>>
    where
        A: Axis + Sync,
        V: Sync,
        S: Sync,
        <A as Axis>::BinInterval: Send,
    {
        self.values.par_iter().map(move |(index, value)| Item {
            index: *index,
            bin: self
                .axes
                .bin(*index)
                .expect("We only iterate over valid indices."),
            value,
        })
    }

    /// An [mutable rayon parallel iterator](rayon::iter::ParallelIterator) over bin indices, bin interval and bin values.
    ///
    /// It only iterates over filled bins in the sparse histogram.
    /// This requires the "rayon" [crate feature](index.html#crate-feature-flags) to be enabled.
    #[cfg(feature = "rayon")]
    pub fn par_iter_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = Item<<A as Axis>::BinInterval, &mut V>>
    where
        A: Axis + Sync + Send,
        V: Send + Sync,
        <A as Axis>::BinInterval: Send,
    {
        let axes = &self.axes;
        self.values.par_iter_mut().map(move |it| Item {
            index: *it.0,
            bin: axes
                .bin(*it.0)
                .expect("We only iterate over valid indices."),
            value: it.1,
        })
    }
}

impl<A, V, S> From<HashHistogram<A, V, S>> for HashMap<usize, V, S> {
    fn from(value: HashHistogram<A, V, S>) -> Self {
        value.values
    }
}
