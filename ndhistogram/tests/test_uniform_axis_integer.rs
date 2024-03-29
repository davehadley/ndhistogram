use ndhistogram::{
    axis::{Axis, BinInterval, Uniform},
    error::AxisError,
};

#[test]
fn test_uniform_num_bins() {
    let ax = Uniform::with_step_size(3, 0, 2).unwrap();
    assert_eq!(ax.num_bins(), 3 + 2)
}

#[test]
fn test_uniform_integer_get_index() {
    let ax = Uniform::with_step_size(3, -2, 2).unwrap();
    let actual: Vec<usize> = (-3..5).map(|x| ax.index(&x).unwrap()).collect();
    let expected = vec![0, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(expected, actual)
}

#[test]
fn test_uniform_integer_get_bin() {
    let ax = Uniform::with_step_size(3, -2, 2).unwrap();
    let actual: Vec<_> = (0..6).map(|it| ax.bin(it)).collect();
    let expected: Vec<_> = vec![
        Some(BinInterval::underflow(-2)),
        Some(BinInterval::new(-2, 0)),
        Some(BinInterval::new(0, 2)),
        Some(BinInterval::new(2, 4)),
        Some(BinInterval::overflow(4)),
        None,
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_uniform_with_step_size_should_return_error_on_negative_step() {
    assert_eq!(
        Uniform::with_step_size(10, 20.0, -1.0),
        Err(AxisError::InvalidStepSize)
    );
}

#[test]
fn test_uniform_with_step_size_should_return_error_on_zero_step() {
    assert_eq!(
        Uniform::with_step_size(10, 20.0, 0.0),
        Err(AxisError::InvalidStepSize)
    );
}
