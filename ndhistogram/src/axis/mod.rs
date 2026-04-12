//! Axis for ND histograms
//!
//! This module contains implementations of [Axis] that are used to represent the axes of
//! an N-dimensional [Histogram](crate::Histogram).
//!

mod bininterval;

use std::iter::FusedIterator;

pub use bininterval::bininterval::BinInterval;
pub use bininterval::singlevaluebininterval::SingleValueBinInterval;
mod uniformcyclic;
pub use uniformcyclic::UniformCyclic;
mod variablecyclic;
pub use variablecyclic::VariableCyclic;
mod uniform;
pub use uniform::Uniform;
mod uniformnoflow;
pub use uniformnoflow::UniformNoFlow;
mod category;
pub use category::Category;
mod categorynoflow;
pub use categorynoflow::CategoryNoFlow;
mod variable;
pub use variable::Variable;
mod variablenoflow;
pub use variablenoflow::VariableNoFlow;

/// An binned axis corresponding to one dimension of an N-dimensional [Histogram](crate::Histogram).
///
/// An Axis is composed of a map from some coordinate space to linear bin number, and the inverse map.
/// For examples see:
/// - [Uniform],
/// - [UniformNoFlow],
/// - [UniformCyclic],
/// - [Variable],
/// - [VariableNoFlow],
/// - [VariableCyclic],
/// - [Category]
/// - and [CategoryNoFlow].
///
/// Most use cases should be covered by the builtin Axis implementations.
/// However, you may implement the Axis trait if you have specialist needs.
///
/// # Examples
///
/// ## Parity Axis
/// Imagine we wanted an 2-bin axis where even values where mapped to one bin
/// and odd values to another bin. We could implement this with the following:
/// ```rust
/// use ndhistogram::axis::{Axis, AxisIter};
/// use ndhistogram::{ndhistogram, Histogram};
/// enum Parity {
///     Even,
///     Odd
/// }
///
/// struct ParityAxis {}
///
/// impl Axis for ParityAxis {
///
///     type Coordinate = i32;
///     type BinInterval = Parity;
///     type Iter<'a> = AxisIter<'a, Self> where Self: 'a;
///
///     fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
///         if coordinate % 2 == 0 { Some(0) } else { Some(1) }
///     }
///
///    fn num_bins(&self) -> usize {
///         2
///     }
///
///    fn bin(&self, index: usize) -> Option<Self::BinInterval> {
///         if index == 0 { Some(Parity::Even) } else { Some(Parity::Odd) }
///     }
///
///     fn iter(&self) -> Self::Iter<'_> {
///         AxisIter::new(self)
///     }
/// }
///
/// let mut hist = ndhistogram!(ParityAxis{}; i32);
/// hist.fill(&1);
/// hist.fill_with(&2, 4);
/// assert_eq!(hist.value(&1), Some(&1));
/// assert_eq!(hist.value(&2), Some(&4));
/// ```
///
pub trait Axis {
    /// The type representing a location on this axis.
    type Coordinate;
    /// The type of an interval representing the set of Coordinates that correspond to a histogram bin
    type BinInterval;

    /// Iterates over all bins on the axis, yielding `(index, bin_interval)`
    type Iter<'a>: Iterator<Item = (usize, Self::BinInterval)>
    where
        Self: 'a;

    /// Map from coordinate to bin number.
    /// Returns an option as not all valid coordinates are necessarily contained within a bin.
    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize>;
    /// The number of bins in this axis, including underflow and overflow.
    fn num_bins(&self) -> usize;

    /// Map from bin number to axis to the interval covering the range of coordinates that this bin contains.
    /// Returns an option in case an index >= [Axis::num_bins] is given.
    fn bin(&self, index: usize) -> Option<Self::BinInterval>;

    /// An iterator over bin numbers
    fn indices(&self) -> impl Iterator<Item = usize> {
        self.iter().map(|(it, _)| it)
    }

    /// An iterator over bin numbers and bin intervals
    fn iter(&self) -> Self::Iter<'_>;

    /// An iterator over bin intervals.
    fn bins(&self) -> impl Iterator<Item = Self::BinInterval> {
        self.iter().map(|(_, it)| it)
    }

    /// The number of dimensions that this object corresponds to.
    /// For most Axis types this will simply be 1.
    /// However, [Axes](crate::Axes) (i.e. a set of [Axis]) also implement [Axis]
    /// and should return the number of [Axis] that it contains.
    fn num_dim(&self) -> usize {
        1
    }
}

/// Iterator over axis bins by index, yielding `(index, bin_interval)` for all bins.
#[derive(Debug)]
pub struct AxisIter<'a, A: Axis> {
    axis: &'a A,
    range: std::ops::Range<usize>,
}

impl<'a, A: Axis> AxisIter<'a, A> {
    /// Create a interator over Axis bins
    pub fn new(axis: &'a A) -> Self {
        Self {
            axis,
            range: 0..axis.num_bins(),
        }
    }
}

impl<'a, A: Axis> Iterator for AxisIter<'a, A> {
    type Item = (usize, A::BinInterval);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.range.next() {
            Some((
                index,
                self.axis
                    .bin(index)
                    .expect("indices() should only produce valid indices"),
            ))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<'a, A: Axis> ExactSizeIterator for AxisIter<'a, A> {
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl<'a, A: Axis> FusedIterator for AxisIter<'a, A> {}
