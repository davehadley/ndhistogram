use ndhistogram::{
    axis::Uniform,
    ndhistogram,
    value::{Sum, Variance},
    Histogram,
};
#[test]
fn test_sum_value_fill() {
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0); Sum);
    hist.fill(&0.0);
    assert_eq!(hist.value(&0.0), Some(&Sum::new(1.0)))
}

fn assert_float_eq(left: f64, right: f64) {
    assert!((left - right).abs() < f64::EPSILON)
}

#[test]
fn test_sum_value_error() {
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0); Sum);
    hist.fill(&0.0);
    hist.fill(&0.0);
    let binvalue = hist.value(&0.0).unwrap();
    assert_float_eq(binvalue.get(), 2.0);
    assert_float_eq(binvalue.variance(), 2.0);
    assert_float_eq(binvalue.standard_deviation(), 2.0f64.sqrt());
}
