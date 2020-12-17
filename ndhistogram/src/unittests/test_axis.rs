#[cfg(test)]
use std::vec;

use crate::axis::Axis;
use crate::axis::Uniform;

#[test]
fn test_uniform_numbins() {
    let nbins = 100;
    let ax = Uniform::new(nbins, 0.0, 1.0);
    assert_eq!(ax.numbins(), nbins)
}

#[test]
fn test_uniform_index() {
    let nbins = 5;
    let ax = Uniform::new(nbins, 0.0, 5.0);
    let actual: Vec<usize> = (-2..7).map(|it| ax.index(&(it as f64))).collect();
    let expected = vec![0, 0, 1, 2, 3, 4, 5, 6, 6];
    assert_eq!(expected, actual)
}