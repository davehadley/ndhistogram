use ndhistogram::axis::{Axis, BinInterval, Uniform};

use std::{convert::TryFrom, ops::Range};

#[test]
fn test_uniform_num_bins() {
    let ax = Uniform::new(5, 0.0, 1.0);
    assert_eq!(ax.num_bins(), 5 + 2)
}

#[test]
fn test_uniform_get_index() {
    let ax = Uniform::new(5, 0.0, 1.0);
    let actual: Vec<usize> = (-2..7)
        .map(|x| ax.index(&((x as f64 + 1e-6) / 5.0)).unwrap())
        .collect();
    let expected = vec![0, 0, 1, 2, 3, 4, 5, 6, 6];
    assert_eq!(expected, actual)
}

#[test]
fn test_uniform_get_bin() {
    let ax = Uniform::new(1, 0.0, 1.0);
    let actual: Vec<_> = (0..4).map(|it| ax.bin(it)).collect();
    let expected: Vec<_> = vec![
        Some(BinInterval::underflow(0.0)),
        Some(BinInterval::new(0.0, 1.0)),
        Some(BinInterval::overflow(1.0)),
        None,
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_uniform_get_edges() {
    let ax = Uniform::new(5, 0.0, 1.0);
    let actual: Vec<Range<f64>> = Range::<i32> { start: -2, end: 7 }
        .map(|x| ax.bin(x as usize))
        .filter_map(|x| x)
        .filter_map(|x| Range::try_from(x).ok())
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
        .all(|it| it.0.start - it.1.start.abs() < delta && (it.0.end - it.1.end).abs() < delta));
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
    let actual: Vec<usize> = ax.indices().collect();
    let expected = vec![0, 1, 2, 3, 4, 5, 6];
    assert_eq!(expected, actual);
}

#[test]
fn test_uniform_iterate_items() {
    let ax = Uniform::new(2, 0.0, 2.0);
    let actual: Vec<_> = ax.into_iter().collect();
    let expected: Vec<(usize, _)> = vec![
        (0, BinInterval::underflow(0.0)),
        (1, BinInterval::new(0.0, 1.0)),
        (2, BinInterval::new(1.0, 2.0)),
        (3, BinInterval::overflow(2.0)),
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_uniform_iterate_bin() {
    let ax = Uniform::new(1, 0.0, 1.0);
    let actual: Vec<_> = ax.bins().collect();
    let expected: Vec<_> = vec![
        BinInterval::underflow(0.0),
        BinInterval::new(0.0, 1.0),
        BinInterval::overflow(1.0),
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_iter_axis() {
    let ax = Uniform::new(100, 0.0, 100.0);
    let expected: Vec<_> = ax.iter().collect();
    let actual: Vec<_> = ax.into_iter().collect();
    assert_eq!(expected, actual);
}

#[test]
fn test_negative_axis() {
    let axis = Uniform::new(10, -5.0, 5.0);
    assert_eq!(axis.bin(0), Some(BinInterval::underflow(-5.0)));
    assert_eq!(axis.bin(1), Some(BinInterval::new(-5.0, -4.0)));
    assert_eq!(axis.bin(11), Some(BinInterval::overflow(5.0)));
}
