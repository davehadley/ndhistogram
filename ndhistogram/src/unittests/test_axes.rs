use float_cmp::approx_eq;

use crate::axis::{Axis, Uniform};
use std::ops::Range;

#[test]
fn test_axes_2d_size() {
    let x = Uniform::new(5, 0.0, 1.0);
    let y = Uniform::new(4, 0.0, 1.0);
    assert_eq!((x, y).size(), (5 + 2) * (4 + 2))
}

#[test]
fn test_uniform_numbins() {
    let x = Uniform::new(5, 0.0, 1.0);
    let y = Uniform::new(4, 0.0, 1.0);
    assert_eq!((x, y).numbins(), (5 * 4))
}
