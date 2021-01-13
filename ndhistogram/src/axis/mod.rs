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

type Iter<'a, BinInterval> = Box<dyn Iterator<Item = (usize, BinInterval)> + 'a>;
type Indices = Box<dyn Iterator<Item = usize>>;
type Bins<'a, BinInterval> = Box<dyn Iterator<Item = BinInterval> + 'a>;

/// An binned axis coresponding to one dimension of an N-dimensional [Histogram].1
///
/// An Axis is composed of a map from some coordinate space to linear bin number, and the inverse map.
/// For examples see [Uniform], [UniformNoFlow], [Category] and [CategoryNoFlow].
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
    type Coordinate;
    type BinInterval;

    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize>;
    fn numbins(&self) -> usize;

    fn bin(&self, index: usize) -> Option<Self::BinInterval>;

    fn indices(&self) -> Indices {
        Box::new(0..self.numbins())
    }

    fn iter(&self) -> Iter<'_, Self::BinInterval> {
        Box::new(self.indices().map(move |it| (it, self.bin(it).unwrap())))
    }

    fn bins(&self) -> Bins<'_, Self::BinInterval> {
        Box::new(self.indices().map(move |it| self.bin(it).unwrap()))
    }
}
