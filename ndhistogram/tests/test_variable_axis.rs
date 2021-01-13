use std::f64::NAN;

use ndhistogram::axis::{Axis, Variable};

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
    Variable::<f64>::new(vec![1.0]);
}

#[test]
#[should_panic]
fn test_variable_nan_edges_panics() {
    Variable::<f64>::new(vec![1.0, NAN, 2.0]);
}
