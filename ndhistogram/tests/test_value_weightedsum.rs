use ndhistogram::{axis::Uniform, ndhistogram, value::WeightedSum, Histogram};

fn assert_float_eq(left: f64, right: f64) {
    assert!(
        (left - right).abs() < (1e-6 * (left + right)),
        "left={} does not equal right={}",
        left,
        right
    )
}

#[test]
fn test_weightedsum_value_fill() {
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0); WeightedSum);
    hist.fill(&0.0);
    assert_float_eq(hist.value(&0.0).unwrap().get(), 1.0)
}

#[test]
fn test_weightedsum_value_fill_with() {
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0); WeightedSum);
    hist.fill_with(&0.0, 2.0);
    assert_float_eq(hist.value(&0.0).unwrap().get(), 2.0)
}

#[test]
fn test_weightedsum_value_error() {
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0); WeightedSum);
    hist.fill_with(&0.0, 2.0);
    let binvalue = hist.value(&0.0).unwrap();
    assert_float_eq(binvalue.get(), 2.0);
    assert_float_eq(binvalue.variance(), 4.0);
    assert_float_eq(binvalue.standard_deviation(), 2.0);
}
