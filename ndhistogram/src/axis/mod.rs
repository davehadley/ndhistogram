//! Axis for ND histograms
//!
//! This module contains implementations of [Axis](Axis) that are used to represent the axes of
//! an N-dimensional [Histogram](crate::Histogram).
//!
mod bininterval;
pub use bininterval::bininterval::BinInterval;
pub use bininterval::singlevaluebininterval::SingleValueBinInterval;
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

type Iter<'a, BinInterval> = Box<dyn Iterator<Item = (usize, BinInterval)> + 'a>;
type Indices = Box<dyn Iterator<Item = usize>>;
type Bins<'a, BinInterval> = Box<dyn Iterator<Item = BinInterval> + 'a>;

/// An binned axis coresponding to one dimension of an N-dimensional [Histogram](crate::Histogram).1
///
/// An Axis is composed of a map from some coordinate space to linear bin number, and the inverse map.
/// For examples see [Uniform](crate::axis::Uniform), [UniformNoFlow](crate::axis::UniformNoFlow), [Category](crate::axis::Category) and [CategoryNoFlow](crate::axis::CategoryNoFlow).
/// Most use cases should be covered by the builtin Axis implementations.
/// However, you may implement the Axis trait if you have specialist needs.
///
/// # Examples
///
/// ## Parity Axis
/// Imagine we wanted a
/// ```rust
/// use ndhistogram::axis::Axis;
/// use ndhistogram::{ndhistogram, Histogram};
/// enum Parity {
///     Even,
///     Odd
/// }
///
/// struct ParityAxis {}
///
/// impl Axis for ParityAxis {
///     type Coordinate = i32;
///
///     type BinInterval = Parity;
///
///     fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
///         if coordinate % 2 == 0 { Some(0) } else { Some(1) }
///     }
///
///    fn numbins(&self) -> usize {
///         2
///     }
///
///    fn bin(&self, index: usize) -> Option<Self::BinInterval> {
///         if index == 0 { Some(Parity::Even) } else { Some(Parity::Odd) }
///     }
/// }
///
/// let mut hist = ndhistogram!(ParityAxis{}; i32);
/// hist.fill(&1);
/// hist.fill_weight(&2, 4);
/// assert_eq!(hist.value(&1), Some(&1));
/// assert_eq!(hist.value(&2), Some(&4));
/// ```
///
pub trait Axis {
    /// The type representing a location on this axis.
    type Coordinate;
    /// The type of an interval representing the set of Coordinates that correspond to a histogram bin
    type BinInterval;

    /// Map from coordinate to bin number.
    /// Returns an option as not all valid coordinates are necessarily contained within a bin.
    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize>;
    /// The number of bins in this axis, including underflow and overflow.
    fn numbins(&self) -> usize;

    /// Map from bin number to axis to the interval covering the range of coordinates that this bin contains.
    /// Returns an option in case an index >= [Axis::numbins] is given.
    fn bin(&self, index: usize) -> Option<Self::BinInterval>;

    /// An iterator over bin numbers
    fn indices(&self) -> Indices {
        Box::new(0..self.numbins())
    }

    /// An iterator over bin numbers and bin intervals
    fn iter(&self) -> Iter<'_, Self::BinInterval> {
        Box::new(self.indices().map(move |it| (it, self.bin(it).unwrap())))
    }

    /// An iterator over bin intervals.
    fn bins(&self) -> Bins<'_, Self::BinInterval> {
        Box::new(self.indices().map(move |it| self.bin(it).unwrap()))
    }
}
