use crate::axis::{binrange::SingleValuedBinRange, CategoryNoFlow};
use crate::axis::{Axis, Category};
use std::{convert::TryFrom, ops::Range};

#[test]
fn test_categorynoflow_numbins() {
    let ax = CategoryNoFlow::new(vec!["A", "B", "C"]);
    assert_eq!(ax.numbins(), 3)
}

#[test]
fn test_categorynoflow_get_index() {
    let cats = vec!["A", "B"];
    let ax = CategoryNoFlow::new(cats.clone());
    let actual: Vec<usize> = cats.iter().map(|c| ax.index(&c).unwrap()).collect();
    let expected = vec![0, 1];
    assert_eq!(expected, actual)
}

#[test]
fn test_categorynoflow_get_index_overflow() {
    let cats = vec!["A", "B"];
    let ax = CategoryNoFlow::new(cats);
    assert_eq!(ax.index(&"D"), None)
}

#[test]
fn test_categorynoflow_get_bin() {
    let cats = vec!["A", "B", "C"];
    let ax = CategoryNoFlow::new(cats);
    let actual: Vec<_> = (0..4).map(|it| ax.bin(it)).collect();
    let expected: Vec<_> = vec![
        Some(SingleValuedBinRange::new("A")),
        Some(SingleValuedBinRange::new("B")),
        Some(SingleValuedBinRange::new("C")),
        None,
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_categorynoflow_clone() {
    let ax = CategoryNoFlow::new(vec!["A", "B", "C"]);
    assert_eq!(ax, ax.clone());
}

#[test]
fn test_categorynoflow_debug_display() {
    let ax = CategoryNoFlow::new(vec!["A", "B", "C"]);
    println!("{:?}", ax);
}

#[test]
fn test_categorynoflow_display() {
    let ax = CategoryNoFlow::new(vec!["A", "B", "C"]);
    println!("{}", ax);
    assert_eq!(format!("{}", ax), "{{A}, {B}, {C}, {overflow}}")
}

#[test]
fn test_categorynoflow_iterate_indices() {
    let ax = CategoryNoFlow::new(vec!["A", "B", "C"]);
    let actual: Vec<usize> = ax.indices().collect();
    let expected = vec![0, 1, 2];
    assert_eq!(expected, actual);
}

#[test]
fn test_categorynoflow_iterate_items() {
    let ax = CategoryNoFlow::new(vec!["A", "B"]);
    let actual: Vec<_> = ax.into_iter().collect();
    let expected: Vec<(usize, _)> = vec![
        (0, SingleValuedBinRange::new("A")),
        (1, SingleValuedBinRange::new("B")),
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_categorynoflow_iterate_bin() {
    let ax = CategoryNoFlow::new(vec!["A", "B"]);
    let actual: Vec<_> = ax.bins().collect();
    let expected: Vec<_> = vec![
        SingleValuedBinRange::new("A"),
        SingleValuedBinRange::new("B"),
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_iter_axis() {
    let ax = CategoryNoFlow::new(vec!["A", "B"]);
    let expected: Vec<_> = ax.iter().collect();
    let actual: Vec<_> = ax.into_iter().collect();
    assert_eq!(expected, actual);
}
