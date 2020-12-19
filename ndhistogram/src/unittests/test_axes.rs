use itertools::iproduct;
use std::collections::HashMap;

#[cfg(test)]
use crate::{axes::Axes, axis::Uniform};

#[test]
fn test_axes1d_uniform() {
    let nbins = 5;
    let ax = (Uniform::new(nbins, 0.0, 5.0),);
    let actual: Vec<usize> = (-2..7).map(|it| ax.index(&(it as f64))).collect();
    let expected = vec![0, 0, 1, 2, 3, 4, 5, 6, 6];
    assert_eq!(expected, actual)
}

#[test]
fn test_axes2d_uniform() {
    let nbins = 2;
    let ax = (
        Uniform::new(nbins, 0.0, 2.0),
        Uniform::new(nbins, 10.0, 12.0),
    );
    let actual: Vec<usize> = iproduct!((9..13), (-1..3))
        .map(|(y, x)| ax.index(&(x as f64, y as f64)))
        .collect();
    let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    assert_eq!(expected, actual)
}
