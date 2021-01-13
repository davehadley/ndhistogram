use std::f64::NAN;

use ndhistogram::axis::{Axis, BinInterval, Variable};

#[test]
fn test_variable_numbins() {
    let ax = Variable::new(vec![0.0, 1.0, 4.0, 8.0]);
    assert_eq!(ax.numbins(), 3 + 2)
}

#[test]
#[should_panic]
fn test_variable_noedges_panics() {
    Variable::<f64>::new(vec![]);
}

#[test]
#[should_panic]
fn test_variable_oneedges_panics() {
    Variable::new(vec![1.0]);
}

#[test]
#[should_panic]
fn test_variable_nan_edges_panics() {
    Variable::new(vec![1.0, NAN, 2.0]);
}

#[test]
fn test_variable_low() {
    let ax = Variable::new(vec![0, 1, 4, 8]);
    assert_eq!(ax.low(), &0)
}

#[test]
fn test_variable_high() {
    let ax = Variable::new(vec![0, 1, 4, 8]);
    assert_eq!(ax.high(), &8)
}

#[test]
fn test_variable_get_index() {
    let ax = Variable::new(vec![0, 1, 4, 8]);
    let actual: Vec<_> = (-1..10).map(|coord| ax.index(&coord).unwrap()).collect();
    let expected = vec![0, 1, 2, 2, 2, 3, 3, 3, 3, 4, 4];
    assert_eq!(actual, expected);
}

#[test]
fn test_variable_get_bin() {
    let ax = Variable::new(vec![0, 1, 4, 8]);
    let actual: Vec<_> = (0..6).map(|index| ax.bin(index)).collect();
    let expected = vec![
        Some(BinInterval::underflow(0)),
        Some(BinInterval::new(0, 1)),
        Some(BinInterval::new(1, 4)),
        Some(BinInterval::new(4, 8)),
        Some(BinInterval::overflow(8)),
        None,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn test_variable_clone() {
    let ax = Variable::new(vec![0, 1, 4, 8]);
    assert_eq!(ax.clone(), ax)
}

#[test]
fn test_variable_debug_display() {
    let ax = Variable::new(vec![0, 1, 4, 8]);
    println!("{:?}", ax);
}

#[test]
fn test_variable_display() {
    let ax = Variable::new(vec![0, 1, 4, 8]);
    println!("{}", ax);
}

#[test]
fn test_variable_iterate_indices() {
    let ax = Variable::new(vec![0, 1, 4, 8]);
    let actual: Vec<_> = ax.indices().collect();
    let expected = vec![0, 1, 2, 3, 4];
    assert_eq!(actual, expected)
}

#[test]
fn test_variable_iterate_bins() {
    let ax = Variable::new(vec![0, 1, 4, 8]);
    let actual: Vec<_> = ax.bins().collect();
    let expected = vec![
        BinInterval::underflow(0),
        BinInterval::new(0, 1),
        BinInterval::new(1, 4),
        BinInterval::new(4, 8),
        BinInterval::overflow(8),
    ];
    assert_eq!(actual, expected)
}

#[test]
fn test_variable_iterate_items() {
    let ax = Variable::new(vec![0, 1, 4, 8]);
    let actual: Vec<_> = ax.into_iter().collect();
    let expected: Vec<_> = ax.indices().zip(ax.bins()).collect();
    assert_eq!(actual, expected)
}
