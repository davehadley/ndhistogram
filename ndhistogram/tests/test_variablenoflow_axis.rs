use std::f64::NAN;

use ndhistogram::{
    axis::{Axis, BinInterval, VariableNoFlow},
    Error,
};

#[test]
fn test_variablenoflow_num_bins() {
    let ax = VariableNoFlow::new(vec![0.0, 1.0, 4.0, 8.0]).unwrap();
    assert_eq!(ax.num_bins(), 3)
}

#[test]
fn test_variablenoflow_noedges_panics() {
    assert_eq!(
        VariableNoFlow::<f64>::new(vec![]),
        Err(Error::InvalidNumberOfBinEdges)
    );
}

#[test]
fn test_variablenoflow_oneedges_panics() {
    assert_eq!(
        VariableNoFlow::new(vec![1.0]),
        Err(Error::InvalidNumberOfBinEdges)
    );
}

#[test]
fn test_variablenoflow_nan_edges_panics() {
    assert_eq!(
        VariableNoFlow::new(vec![1.0, NAN, 2.0]),
        Err(Error::FailedToSortBinEdges)
    );
}

#[test]
fn test_variablenoflow_low() {
    let ax = VariableNoFlow::new(vec![0, 1, 4, 8]).unwrap();
    assert_eq!(ax.low(), &0)
}

#[test]
fn test_variablenoflow_high() {
    let ax = VariableNoFlow::new(vec![0, 1, 4, 8]).unwrap();
    assert_eq!(ax.high(), &8)
}

#[test]
fn test_variablenoflow_get_index() {
    let ax = VariableNoFlow::new(vec![0, 1, 4, 8]).unwrap();
    let actual: Vec<_> = (-1..10).filter_map(|coord| ax.index(&coord)).collect();
    let expected = vec![0, 1, 1, 1, 2, 2, 2, 2];
    assert_eq!(actual, expected);
}

#[test]
fn test_variablenoflow_get_bin() {
    let ax = VariableNoFlow::new(vec![0, 1, 4, 8]).unwrap();
    let actual: Vec<_> = (0..4).map(|index| ax.bin(index)).collect();
    let expected = vec![
        Some(BinInterval::new(0, 1)),
        Some(BinInterval::new(1, 4)),
        Some(BinInterval::new(4, 8)),
        None,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn test_variablenoflow_clone() {
    let ax = VariableNoFlow::new(vec![0, 1, 4, 8]);
    assert_eq!(ax.clone(), ax)
}

#[test]
fn test_variablenoflow_debug_display() {
    let ax = VariableNoFlow::new(vec![0, 1, 4, 8]);
    println!("{:?}", ax);
}

#[test]
fn test_variablenoflow_display() {
    let ax = VariableNoFlow::new(vec![0, 1, 4, 8]).unwrap();
    println!("{}", ax);
}

#[test]
fn test_variablenoflow_iterate_indices() {
    let ax = VariableNoFlow::new(vec![0, 1, 4, 8]).unwrap();
    let actual: Vec<_> = ax.indices().collect();
    let expected = vec![0, 1, 2];
    assert_eq!(actual, expected)
}

#[test]
fn test_variablenoflow_iterate_bins() {
    let ax = VariableNoFlow::new(vec![0, 1, 4, 8]).unwrap();
    let actual: Vec<_> = ax.bins().collect();
    let expected = vec![
        BinInterval::new(0, 1),
        BinInterval::new(1, 4),
        BinInterval::new(4, 8),
    ];
    assert_eq!(actual, expected)
}

#[test]
fn test_variablenoflow_iterate_items() {
    let ax = VariableNoFlow::new(vec![0, 1, 4, 8]).unwrap();
    let actual: Vec<_> = ax.into_iter().collect();
    let expected: Vec<_> = ax.indices().zip(ax.bins()).collect();
    assert_eq!(actual, expected)
}

#[test]
fn test_negative_axis_index() -> Result<(), Error> {
    let axis = VariableNoFlow::new(vec![-3, -1, 0, 1])?;
    let actual: Vec<_> = (-4..3).filter_map(|loc| axis.index(&loc)).collect();
    let expected = vec![0, 0, 1, 2];
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_negative_axis_bin() -> Result<(), Error> {
    let axis = VariableNoFlow::new(vec![-3, -1, 0, 1])?;
    let actual: Vec<_> = (0..4).map(|index| axis.bin(index)).collect();
    let expected = vec![
        Some(BinInterval::new(-3, -1)),
        Some(BinInterval::new(-1, 0)),
        Some(BinInterval::new(0, 1)),
        None,
    ];
    assert_eq!(actual, expected);
    Ok(())
}
