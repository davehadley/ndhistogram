use ndhistogram::{axis::Uniform, ndhistogram, value::Sum, FillWith, Histogram};
#[test]
fn test_sum_value_fill() {
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0).unwrap(); Sum<i32>);
    hist.fill(&0.0);
    assert_eq!(hist.value(&0.0).unwrap().get(), 1)
}

fn assert_float_eq(left: f64, right: f64) {
    assert!((left - right).abs() < f64::EPSILON)
}

#[test]
fn test_sum_value_error() {
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0).unwrap(); Sum);
    hist.fill(&0.0);
    hist.fill(&0.0);
    let binvalue = hist.value(&0.0).unwrap();
    assert_float_eq(binvalue.get(), 2.0);
    assert_float_eq(binvalue.variance(), 2.0);
    assert_float_eq(binvalue.standard_deviation(), 2.0f64.sqrt());
}

#[test]
fn test_sum_value_fill_with() {
    let mut v = Sum::new();
    v.fill_with(12345.678);
    assert_float_eq(v.get(), 12345.678);
}
