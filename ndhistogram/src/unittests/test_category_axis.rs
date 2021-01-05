use crate::axis::binrange::SingleValuedBinRange;
use crate::axis::{Axis, Category};
use std::{convert::TryFrom, ops::Range};

fn category_ab() -> (Vec<String>, Category<String>) {
    let cats: Vec<String> = vec![String::from("A"), String::from("B")];
    (cats.clone(), Category::new(cats))
}

fn category_abc() -> (Vec<String>, Category<String>) {
    let cats: Vec<String> = vec![String::from("A"), String::from("B"), String::from("C")];
    (cats.clone(), Category::new(cats))
}

#[test]
fn test_category_numbins() {
    let (_, ax) = category_abc();
    assert_eq!(ax.numbins(), 3 + 1)
}

#[test]
fn test_category_get_index() {
    let (cats, ax) = category_ab();
    let actual: Vec<usize> = cats.iter().map(|c| ax.index(c).unwrap()).collect();
    let expected = vec![0, 1];
    assert_eq!(expected, actual)
}

#[test]
fn test_category_get_index_overflow() {
    let cats = ["A", "B"];
    let ax = Category::new(&cats);
    assert_eq!(ax.index(&"D").unwrap(), 2)
}

#[test]
fn test_category_get_bin() {
    let cats = ["A", "B", "C"];
    let ax = Category::new(&cats);
    let actual: Vec<_> = (0..5).map(|it| ax.bin(it)).collect();
    let expected: Vec<_> = vec![
        Some(SingleValuedBinRange::new(&"A")),
        Some(SingleValuedBinRange::new(&"B")),
        Some(SingleValuedBinRange::new(&"C")),
        Some(SingleValuedBinRange::overflow()),
        None,
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_category_clone() {
    let ax = Category::new(&["A", "B", "C"]);
    assert_eq!(ax, ax.clone());
}

#[test]
fn test_category_debug_display() {
    let ax = Category::new(&["A", "B", "C"]);
    println!("{:?}", ax);
}

// #[test]
// fn test_category_display() {
//     let ax = Category::new(5, 0.0, 1.0);
//     println!("{}", ax);
// }

// #[test]
// fn test_category_iterate_indices() {
//     let ax = Category::new(5, 0.0, 1.0);
//     let actual: Vec<usize> = ax.indices().collect();
//     let expected = vec![0, 1, 2, 3, 4, 5, 6];
//     assert_eq!(expected, actual);
// }

// #[test]
// fn test_category_iterate_items() {
//     let ax = Category::new(2, 0.0, 2.0);
//     let actual: Vec<_> = ax.into_iter().collect();
//     let expected: Vec<(usize, _)> = vec![
//         (0, ContinuousBinRange::underflow(0.0)),
//         (1, ContinuousBinRange::new(0.0, 1.0)),
//         (2, ContinuousBinRange::new(1.0, 2.0)),
//         (3, ContinuousBinRange::overflow(2.0)),
//     ];
//     assert_eq!(expected, actual);
// }

// #[test]
// fn test_category_iterate_bin() {
//     let ax = Category::new(1, 0.0, 1.0);
//     let actual: Vec<_> = ax.bins().collect();
//     let expected: Vec<_> = vec![
//         ContinuousBinRange::underflow(0.0),
//         ContinuousBinRange::new(0.0, 1.0),
//         ContinuousBinRange::overflow(1.0),
//     ];
//     assert_eq!(expected, actual);
// }

// #[test]
// fn test_iter_axis() {
//     let ax = Category::new(100, 0.0, 100.0);
//     let expected: Vec<_> = ax.iter().collect();
//     let actual: Vec<_> = ax.into_iter().collect();
//     assert_eq!(expected, actual);
// }
