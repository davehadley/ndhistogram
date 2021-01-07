use float_cmp::approx_eq;

use crate::axis::{Axis, Category, Uniform};
use std::ops::Range;

#[test]
fn test_axes_1d_numbins() {
    let x = Uniform::new(5, 0.0, 1.0);
    let xtuple = (x.clone(),);
    assert_eq!(xtuple.numbins(), x.numbins())
}

#[test]
fn test_axes_2d_numbins() {
    let x = Uniform::new(5, 0.0, 1.0);
    let y = Uniform::new(4, 0.0, 1.0);
    let xy = (x.clone(), y.clone());
    assert_eq!(xy.numbins(), x.numbins() * y.numbins())
}

// #[test]
// fn test_axes_3d_numbins() {
//     let x = Uniform::new(5, 0.0, 1.0);
//     let y = Uniform::new(4, 0.0, 1.0);
//     let z = Category::new(vec!["A", "B", "C"]);
//     assert_eq!((x, y, z).numbins(), x.numbins() * y.numbins() * z.numbins());
// }
