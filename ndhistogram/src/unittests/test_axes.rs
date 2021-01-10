use float_cmp::approx_eq;

use crate::axis::{
    binrange::{ContinuousBinRange, SingleValuedBinRange},
    Axis, Category, CategoryNoFlow, Uniform,
};
use std::ops::Range;

#[test]
fn test_axes_2d_numbins() {
    let x = Uniform::new(5, 0.0, 1.0);
    let y = Uniform::new(4, 0.0, 1.0);
    assert_eq!((x, y).numbins(), (5 + 2) * (4 + 2))
}

#[test]
fn test_axes_2d_iter() {
    let x = Category::new(vec!["A", "B"]);
    let y = Category::new(vec!["C", "D"]);
    let actual: Vec<_> = (x, y).iter().collect();
    let new = SingleValuedBinRange::new;
    let overflow = SingleValuedBinRange::overflow;
    let expected = vec![
        (0, (new("A"), new("C"))),
        (1, (new("B"), new("C"))),
        (2, (overflow(), new("C"))),
        (3, (new("A"), new("D"))),
        (4, (new("B"), new("D"))),
        (5, (overflow(), new("D"))),
        (6, (new("A"), overflow())),
        (7, (new("B"), overflow())),
        (8, (overflow(), overflow())),
    ];
    assert_eq!(expected, actual)
}

#[test]
fn test_axes_2d_index() {
    let x = CategoryNoFlow::new(vec!["A", "B"]);
    let y = CategoryNoFlow::new(vec!["C", "D"]);
    let xy = (x.clone(), y.clone());
    let actual = vec![
        xy.index(&("A", "C")),
        xy.index(&("B", "C")),
        xy.index(&("A", "D")),
        xy.index(&("B", "D")),
    ];
    let expected: Vec<_> = (0..4).map(Some).collect();
    assert_eq!(expected, actual)
}

#[test]
fn test_axes_2d_bin() {
    let x = CategoryNoFlow::new(vec!["A", "B"]);
    let y = CategoryNoFlow::new(vec!["C", "D"]);
    let xy = (x.clone(), y.clone());
    let actual = vec![xy.bin(0), xy.bin(1), xy.bin(2), xy.bin(3)];
    let new = SingleValuedBinRange::new;
    let expected = vec![
        Some((new("A"), new("C"))),
        Some((new("B"), new("C"))),
        Some((new("A"), new("D"))),
        Some((new("B"), new("D"))),
    ];
    assert_eq!(expected, actual)
}

#[test]
fn test_axes_3d_numbins() {
    let x = Uniform::new(5, 0.0, 1.0);
    let y = Uniform::new(4, 0.0, 1.0);
    let z = Uniform::new(3, 0.0, 1.0);
    assert_eq!((x, y, z).numbins(), (5 + 2) * (4 + 2) * (3 + 2))
}

#[test]
fn test_axes_3d_iter() {
    let x = CategoryNoFlow::new(vec!["x0", "x1"]);
    let y = CategoryNoFlow::new(vec!["y0", "y1"]);
    let z = CategoryNoFlow::new(vec!["z0", "z1"]);
    let actual: Vec<_> = (x, y, z).iter().collect();
    let new = SingleValuedBinRange::new;
    let expected = vec![
        (0, (new("x0"), new("y0"), new("z0"))),
        (1, (new("x1"), new("y0"), new("z0"))),
        (2, (new("x0"), new("y1"), new("z0"))),
        (3, (new("x1"), new("y1"), new("z0"))),
        (4, (new("x0"), new("y0"), new("z1"))),
        (5, (new("x1"), new("y0"), new("z1"))),
        (6, (new("x0"), new("y1"), new("z1"))),
        (7, (new("x1"), new("y1"), new("z1"))),
    ];
    assert_eq!(expected, actual)
}

#[test]
fn test_axes_3d_index() {
    let x = CategoryNoFlow::new(vec!["x0", "x1"]);
    let y = CategoryNoFlow::new(vec!["y0", "y1"]);
    let z = CategoryNoFlow::new(vec!["z0", "z1"]);
    let xyz = (x.clone(), y.clone(), z.clone());
    let actual = vec![
        xyz.index(&("x0", "y0", "z0")),
        xyz.index(&("x1", "y0", "z0")),
        xyz.index(&("x0", "y1", "z0")),
        xyz.index(&("x1", "y1", "z0")),
        xyz.index(&("x0", "y0", "z1")),
        xyz.index(&("x1", "y0", "z1")),
        xyz.index(&("x0", "y1", "z1")),
        xyz.index(&("x1", "y1", "z1")),
    ];
    let expected: Vec<_> = (0..8).map(Some).collect();
    assert_eq!(expected, actual)
}

#[test]
fn test_axes_3d_bin() {
    let x = CategoryNoFlow::new(vec!["x0", "x1"]);
    let y = CategoryNoFlow::new(vec!["y0", "y1"]);
    let z = CategoryNoFlow::new(vec!["z0", "z1"]);
    let xyz = (x.clone(), y.clone(), z.clone());
    let actual: Vec<_> = (0..8).map(|index| xyz.bin(index)).collect();
    let new = SingleValuedBinRange::new;
    let expected = vec![
        Some((new("x0"), new("y0"), new("z0"))),
        Some((new("x1"), new("y0"), new("z0"))),
        Some((new("x0"), new("y1"), new("z0"))),
        Some((new("x1"), new("y1"), new("z0"))),
        Some((new("x0"), new("y0"), new("z1"))),
        Some((new("x1"), new("y0"), new("z1"))),
        Some((new("x0"), new("y1"), new("z1"))),
        Some((new("x1"), new("y1"), new("z1"))),
    ];
    assert_eq!(expected, actual)
}
