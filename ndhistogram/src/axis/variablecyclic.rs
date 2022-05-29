use super::{Axis, BinInterval, Variable};
use std::fmt::Debug; // TODO Display

use num_traits::Num;
use serde::{Deserialize, Serialize};

/// A wrap-around axis with variable-sized bins.
///
/// A wrap-around axis with variable-sized bins, constructed from a list of bin
/// edges.
///
/// TODO complete docs, including examples

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct VariableCyclic<T = f64> {
    axis: Variable<T>,
}

impl<T> VariableCyclic<T>
where
    T: PartialOrd + Copy,
{
    /// Create a wrap-around axis with [Variable] binning given a set of bin edges.
    ///
    /// # Panics
    ///
    /// Panics if fewer than 2 edges are provided, or if the edges cannot be
    /// sorted (for example when given NAN).
    pub fn new<I: IntoIterator<Item = T>>(bin_edges: I) -> Self {
        Self {
            axis: Variable::new(bin_edges),
        }
    }

    /// Low edge of axis (excluding wrap-around)
    #[inline]
    pub fn low(&self) -> &T {
        self.axis.low()
    }

    /// High edge of axis (excluding wrap-around)
    #[inline]
    pub fn high(&self) -> &T {
        self.axis.high()
    }
}

impl<T> Axis for VariableCyclic<T>
where
    T: PartialOrd + Copy + Num,
{
    type Coordinate = T;
    type BinInterval = BinInterval<T>;

    #[inline]
    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
        // TODO can we sensibly avoid copy-paste code reuse with UniformCyclic::index ?
        let (mut x, hi, lo) = (*coordinate, *self.axis.high(), *self.axis.low());
        let range = hi - lo;
        x = (x - lo) % range;
        if x < T::zero() {
            x = range + x;
        }
        x = x + lo;
        self.axis.index(&x).map(|n| n - 1)
    }

    #[inline]
    fn num_bins(&self) -> usize {
        self.axis.num_bins() - 2
    }

    #[inline]
    fn bin(&self, index: usize) -> Option<Self::BinInterval> {
        self.axis.bin(index + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn bint<T>(lo: T, hi: T) -> Option<BinInterval<T>> {
        Some(BinInterval::new(lo, hi))
    }

    #[rstest(/**/ bin_no, expected_interval, edges,
             case(0, bint(0.0, 0.5), &[0.0, 0.5, 2.0]),
             case(0, bint(0.0, 0.5), &[0.5, 2.0, 0.0]),
             case(1, bint(0.5, 2.0), &[0.0, 0.5, 2.0]),
             case(1, bint(0.5, 2.0), &[0.5, 2.0, 0.0]),
    )]
    fn bin_float(bin_no: usize, expected_interval: Option<BinInterval<f32>>, edges: &[f32]) {
        let axis = VariableCyclic::new(edges.iter().cloned());
        assert_eq!(axis.bin(bin_no), expected_interval);
    }

    #[rstest(/**/ bin_no, expected_interval, edges,
             case(0, bint(0,  5), &[ 0,  5, 20]),
             case(0, bint(0,  5), &[ 5, 20,  0]),
             case(1, bint(5, 20), &[ 0,  5, 20]),
             case(1, bint(5, 20), &[ 5, 20,  0]),
    )]
    fn bin_int(bin_no: usize, expected_interval: Option<BinInterval<i32>>, edges: &[i32]) {
        let axis = VariableCyclic::new(edges.iter().cloned());
        assert_eq!(axis.bin(bin_no), expected_interval);
    }

    #[rstest(/**/coordinate, expected_index,          edges,
             case(0.5      ,     Some(0)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
             case(1.5      ,     Some(1)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
             case(3.0      ,     Some(2)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
             case(6.0      ,     Some(3)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
             case(0.5 + 8.0,     Some(0)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
             case(1.5 + 8.0,     Some(1)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
             case(3.0 + 8.0,     Some(2)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
             case(6.0 + 8.0,     Some(3)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
             case(0.5 - 8.0,     Some(0)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
             case(1.5 - 8.0,     Some(1)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
             case(3.0 - 8.0,     Some(2)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
             case(6.0 - 8.0,     Some(3)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
    )]
    fn float_index(coordinate: f32, expected_index: Option<usize>, edges: &[f32]) {
        let axis = VariableCyclic::new(edges.iter().cloned());
        assert_eq!(axis.index(&coordinate), expected_index);
    }

    #[rstest(/**/coordinate, expected_index,          edges,
             case( 5       ,     Some(0)   , &[ 0, 10, 20, 40, 80]),
             case(15       ,     Some(1)   , &[ 0, 10, 20, 40, 80]),
             case(30       ,     Some(2)   , &[ 0, 10, 20, 40, 80]),
             case(60       ,     Some(3)   , &[ 0, 10, 20, 40, 80]),
             case( 5 + 80  ,     Some(0)   , &[ 0, 10, 20, 40, 80]),
             case(15 + 80  ,     Some(1)   , &[ 0, 10, 20, 40, 80]),
             case(30 + 80  ,     Some(2)   , &[ 0, 10, 20, 40, 80]),
             case(60 + 80  ,     Some(3)   , &[ 0, 10, 20, 40, 80]),
             case( 5 - 80  ,     Some(0)   , &[ 0, 10, 20, 40, 80]),
             case(15 - 80  ,     Some(1)   , &[ 0, 10, 20, 40, 80]),
             case(30 - 80  ,     Some(2)   , &[ 0, 10, 20, 40, 80]),
             case(60 - 80  ,     Some(3)   , &[ 0, 10, 20, 40, 80]),
    )]
    fn int_index(coordinate: i32, expected_index: Option<usize>, edges: &[i32]) {
        let axis = VariableCyclic::new(edges.iter().cloned());
        assert_eq!(axis.index(&coordinate), expected_index);
    }

    #[rstest(/**/ edges,
             case(&[1.0, 2.0]),
             case(&[8.5, 2.3, 9.4, -23.4]),
    )]
    fn indices(edges: &[f32]) {
        let n = edges.len() - 1;
        let axis = VariableCyclic::new(edges.iter().cloned());
        let indices = axis.indices().collect::<Vec<_>>();
        assert_eq!(indices, (0..n).collect::<Vec<_>>());
    }
}

#[cfg(test)]
mod test_histogram {
    use super::*;
    use crate::{ndhistogram, Histogram};

    #[test]
    fn wrap_float_fill() {
        let mut hist = ndhistogram!(VariableCyclic::new(vec![8.0, 0.0, 4.0, 2.0]); u8);
        let v = &5.0;
        let r = 8.0;
        hist.fill(v);
        hist.fill(&(v + r));
        hist.fill(&(v - r));
        assert_eq!(hist.value(v), Some(&3));
        assert_eq!(hist.value_at_index(2), Some(&3));
    }

    #[test]
    fn wrap_int_fill() {
        let mut hist = ndhistogram!(VariableCyclic::new(vec![8, 0, 4, 2]); u8);
        let v = &5;
        let r = 8;
        hist.fill(v);
        hist.fill(&(v + r));
        hist.fill(&(v - r));
        assert_eq!(hist.value(v), Some(&3));
        assert_eq!(hist.value_at_index(2), Some(&3));
    }

    #[test]
    fn wrap_float_value() {
        let mut hist = ndhistogram!(VariableCyclic::new(vec![4.0, 8.0, 2.0, 1.0]); u8);
        let v = &2.3;
        let r = 7.0;
        hist.fill(v);
        assert_eq!(hist.value(v), Some(&1));
        assert_eq!(hist.value(&(v - r)), Some(&1));
        assert_eq!(hist.value(&(v + r)), Some(&1));
    }
}
