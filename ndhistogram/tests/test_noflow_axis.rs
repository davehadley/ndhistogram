use ndhistogram::axis::{bininterval::BinInterval, Axis, UniformNoFlow};

#[test]
fn test_uniformnoflow_axis_numbins() {
    let axis = UniformNoFlow::new(2, 0.0, 2.0);
    assert_eq!(axis.numbins(), 2);
}

#[test]
fn test_uniformnoflow_axis_index() {
    let axis = UniformNoFlow::new(2, 0.0, 2.0);
    assert_eq!(axis.numbins(), 2);
}

#[test]
fn test_uniformnoflow_get_index() {
    let ax = UniformNoFlow::new(2, 0.0, 2.0);
    let actual: Vec<_> = (-1..3).map(|x| ax.index(&(x as f64))).collect();
    let expected = vec![None, Some(0), Some(1), None];
    assert_eq!(expected, actual)
}

#[test]
fn test_uniformnoflow_get_bin() {
    let ax = UniformNoFlow::new(2, 0.0, 2.0);
    let actual: Vec<_> = (0..3).map(|it| ax.bin(it)).collect();
    let expected: Vec<_> = vec![
        Some(BinInterval::new(0.0, 1.0)),
        Some(BinInterval::new(1.0, 2.0)),
        None,
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_uniform_noflow_indices() {
    let ax = UniformNoFlow::new(2, 0.0, 2.0);
    let actual: Vec<_> = ax.indices().collect();
    assert_eq!(actual, vec![0, 1]);
}

#[test]
fn test_uniform_noflow_bins() {
    let ax = UniformNoFlow::new(2, 0.0, 2.0);
    let actual: Vec<_> = ax.bins().collect();
    assert_eq!(
        actual,
        vec![BinInterval::new(0.0, 1.0), BinInterval::new(1.0, 2.0)]
    );
}

#[test]
fn test_uniform_noflow_iter() {
    let ax = UniformNoFlow::new(2, 0.0, 2.0);
    let actual: Vec<_> = ax.into_iter().collect();
    assert_eq!(
        actual,
        vec![
            (0, BinInterval::new(0.0, 1.0)),
            (1, BinInterval::new(1.0, 2.0))
        ]
    );
}

#[test]
fn test_uniform_noflow_display() {
    let ax = UniformNoFlow::new(2, 0.0, 2.0);
    format!("{}", ax);
}

#[test]
fn test_uniform_noflow_debug() {
    let ax = UniformNoFlow::new(2, 0.0, 2.0);
    format!("{:?}", ax);
}

#[test]
fn test_uniform_noflow_clone() {
    let ax1 = UniformNoFlow::new(2, 0.0, 2.0);
    let ax2 = ax1.clone();
    assert_eq!(ax1, ax2);
}
