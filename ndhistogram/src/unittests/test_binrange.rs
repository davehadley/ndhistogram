use crate::axis::binrange::ContinuousBinRange;

#[test]
fn test_binrange_equality() {
    let underflow = ContinuousBinRange::underflow(0.0);
    let overflow = ContinuousBinRange::overflow(1.0);
    let bin = ContinuousBinRange::new(0.0, 1.0);
}
