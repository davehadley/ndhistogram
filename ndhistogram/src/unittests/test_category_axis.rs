use crate::axis::binrange::SingleValuedBinRange;
use crate::axis::{Axis, Category};
use std::{convert::TryFrom, ops::Range};

#[test]
fn test_category_numbins() {
    let ax = Category::new(vec!["A", "B", "C"]);
    assert_eq!(ax.numbins(), 3 + 1)
}

#[test]
fn test_category_get_index() {
    let cats = vec!["A", "B"];
    let ax = Category::new(cats.clone());
    let actual: Vec<usize> = cats.iter().map(|c| ax.index(&c).unwrap()).collect();
    let expected = vec![0, 1];
    assert_eq!(expected, actual)
}

#[test]
fn test_category_get_index_overflow() {
    let cats = vec!["A", "B"];
    let ax = Category::new(cats);
    assert_eq!(ax.index(&"D").unwrap(), 2)
}

#[test]
fn test_category_get_bin() {
    let cats = vec!["A", "B", "C"];
    let ax = Category::new(cats);
    let actual: Vec<_> = (0..5).map(|it| ax.bin(it)).collect();
    let expected: Vec<_> = vec![
        Some(SingleValuedBinRange::new("A")),
        Some(SingleValuedBinRange::new("B")),
        Some(SingleValuedBinRange::new("C")),
        Some(SingleValuedBinRange::overflow()),
        None,
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_category_clone() {
    let ax = Category::new(vec!["A", "B", "C"]);
    assert_eq!(ax, ax.clone());
}

#[test]
fn test_category_debug_display() {
    let ax = Category::new(vec!["A", "B", "C"]);
    println!("{:?}", ax);
}

#[test]
fn test_category_display() {
    let ax = Category::new(vec!["A", "B", "C"]);
    println!("{}", ax);
    assert_eq!(format!("{}", ax), "{{A}, {B}, {C}, {overflow}}")
}

#[test]
fn test_category_iterate_indices() {
    let ax = Category::new(vec!["A", "B", "C"]);
    let actual: Vec<usize> = ax.indices().collect();
    let expected = vec![0, 1, 2, 3];
    assert_eq!(expected, actual);
}

#[test]
fn test_category_iterate_items() {
    let ax = Category::new(vec!["A", "B"]);
    let actual: Vec<_> = ax.into_iter().collect();
    let expected: Vec<(usize, _)> = vec![
        (0, SingleValuedBinRange::new("A")),
        (1, SingleValuedBinRange::new("B")),
        (2, SingleValuedBinRange::overflow()),
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_category_iterate_bin() {
    let ax = Category::new(vec!["A", "B"]);
    let actual: Vec<_> = ax.bins().collect();
    let expected: Vec<_> = vec![
        SingleValuedBinRange::new("A"),
        SingleValuedBinRange::new("B"),
        SingleValuedBinRange::overflow(),
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_iter_axis() {
    let ax = Category::new(vec!["A", "B"]);
    let expected: Vec<_> = ax.iter().collect();
    let actual: Vec<_> = ax.into_iter().collect();
    assert_eq!(expected, actual);
}

fn string_category_ab() -> (Vec<String>, Category<String>) {
    let cats: Vec<String> = vec![String::from("A"), String::from("B")];
    (cats.clone(), Category::new(cats))
}

fn string_category_abc() -> (Vec<String>, Category<String>) {
    let cats: Vec<String> = vec![String::from("A"), String::from("B"), String::from("C")];
    (cats.clone(), Category::new(cats))
}

#[test]
fn test_string_category() {
    // sanaity check that we get sensible results when category is String type
    let (cats, category) = string_category_ab();
    assert_eq!(category.numbins(), cats.len() + 1);
    assert_eq!(category.indices().collect::<Vec<_>>(), vec![0, 1, 2]);
    assert_eq!(category.index(&"A".to_string()), Some(0));
    assert_eq!(
        category.bin(0),
        Some(SingleValuedBinRange::new(String::from("A")))
    );
    assert_eq!(
        category.bins().collect::<Vec<_>>(),
        vec![
            SingleValuedBinRange::new(String::from("A")),
            SingleValuedBinRange::new(String::from("B")),
            SingleValuedBinRange::overflow()
        ]
    );
}
