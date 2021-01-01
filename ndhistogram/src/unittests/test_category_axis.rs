use crate::axis::binrange::SingleValuedBinRange;
use crate::axis::{Axis, Category};
use std::{convert::TryFrom, ops::Range};

#[test]
fn test_category_numbins() {
    let ax = Category::new(&["A", "B", "C"]);
    assert_eq!(ax.numbins(), 3 + 1)
}

#[test]
fn test_category_get_index() {
    let cats: Vec<_> = vec!["A", "B"];
    let ax = Category::<&str>::new(cats.clone());
    let actual: Vec<usize> = cats.iter().map(|c| ax.index(c).unwrap()).collect();
    let expected = vec![1, 2];
    assert_eq!(expected, actual)
}

#[test]
fn test_category_get_index_overflow() {
    let cats = ["A", "B", "C"];
    let ax = Category::new(&cats);
    assert_eq!(ax.index(&"D").unwrap(), 0)
}

#[test]
fn test_category_get_bin() {
    let cats = ["A", "B", "C"];
    let ax = Category::new(&cats);
    let actual: Vec<_> = (0..4).map(|it| ax.bin(it)).collect();
    let expected: Vec<_> = vec![
        Some(SingleValuedBinRange::new(&"A")),
        Some(SingleValuedBinRange::new(&"B")),
        Some(SingleValuedBinRange::new(&"C")),
        Some(SingleValuedBinRange::overflow()),
        None,
    ];
    assert_eq!(expected, actual);
}

// #[test]
// fn test_category_get_edges() {
//     let ax = Category::new(5, 0.0, 1.0);
//     let actual: Vec<Range<f64>> = Range::<i32> { start: -2, end: 7 }
//         .map(|x| ax.bin(x as usize))
//         .filter_map(|x| x)
//         .filter_map(|x| Range::try_from(x).ok())
//         .collect();
//     let edges: Vec<f64> = vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
//     let expected: Vec<Range<f64>> = edges
//         .iter()
//         .zip(edges.iter().skip(1).clone())
//         .map(|it| Range {
//             start: *it.0,
//             end: *it.1,
//         })
//         .collect();
//     assert_eq!(expected.len(), actual.len());
//     let delta = 1.0e-6;
//     assert!(expected
//         .iter()
//         .zip(actual)
//         .all(|it| it.0.start - it.1.start.abs() < delta && (it.0.end - it.1.end).abs() < delta));
// }

// #[test]
// fn test_category_clone() {
//     let ax = Category::new(5, 0.0, 1.0);
//     assert_eq!(ax, ax.clone());
// }

// #[test]
// fn test_category_debug_display() {
//     let ax = Category::new(5, 0.0, 1.0);
//     println!("{:?}", ax);
// }

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
