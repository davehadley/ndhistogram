use float_cmp::approx_eq;

use crate::axis::{Axis, Uniform};
use std::ops::Range;

#[test]
fn test_uniform_size() {
    let ax = Uniform::new(5, 0.0, 1.0);
    assert_eq!(ax.size(), 5 + 2)
}

#[test]
fn test_uniform_numbins() {
    let ax = Uniform::new(5, 0.0, 1.0);
    assert_eq!(ax.numbins(), 5)
}

#[test]
fn test_uniform_get_index() {
    let ax = Uniform::new(5, 0.0, 1.0);
    let actual: Vec<usize> = (-2..7).map(|x| ax.index(&(x as f64 / 5.0))).collect();
    let expected = vec![0, 0, 1, 2, 3, 4, 5, 6, 6];
    assert_eq!(expected, actual)
}

#[test]
fn test_uniform_get_edges() {
    let ax = Uniform::new(5, 0.0, 1.0);
    let actual: Vec<Range<f64>> = Range::<i32> { start: -2, end: 7 }
        .map(|x| ax.bin(x as usize))
        .filter_map(|x| x)
        .collect();
    let edges: Vec<f64> = vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
    let expected: Vec<Range<f64>> = edges
        .iter()
        .zip(edges.iter().skip(1).clone())
        .map(|it| Range {
            start: *it.0,
            end: *it.1,
        })
        .collect();
    assert_eq!(expected.len(), actual.len());
    let delta = 1.0e-6;
    assert!(expected
        .iter()
        .zip(actual)
        .all(|it| (it.0.start - it.1.start).abs() < delta && (it.0.end - it.1.end).abs() < delta));
}

#[test]
fn test_uniform_clone() {
    let ax = Uniform::new(5, 0.0, 1.0);
    assert_eq!(ax, ax.clone());
}

#[test]
fn test_uniform_debug_display() {
    let ax = Uniform::new(5, 0.0, 1.0);
    println!("{:?}", ax);
}

#[test]
fn test_uniform_display() {
    let ax = Uniform::new(5, 0.0, 1.0);
    println!("{}", ax);
}

#[test]
fn test_uniform_iterate_indices() {
    let ax = Uniform::new(5, 0.0, 1.0);
    let actual: Vec<usize> = ax.iter_indices().collect();
    let expected = vec![0, 1, 2, 3, 4, 5, 6];
    assert_eq!(expected, actual);
}
