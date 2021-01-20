use ndhistogram::{axis::Uniform, ndhistogram, value::Mean, Histogram, VecHistogram};

fn assert_float_eq(left: f64, right: f64) {
    assert!(
        (left - right).abs() < (1e-6 * (left + right)),
        format!("left={} does not equal right={}", left, right)
    )
}

fn simple_filled_float_mean_hist() -> VecHistogram<(Uniform,), Mean> {
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0); Mean);
    hist.fill_with(&0.0, 1.0);
    hist.fill_with(&0.0, 2.0);
    hist.fill_with(&0.0, 3.0);
    hist
}

#[test]
fn test_mean_value_mean() {
    let hist = simple_filled_float_mean_hist();
    assert_float_eq(hist.value(&0.0).unwrap().get(), 2.0)
}

#[test]
fn test_mean_value_numsamples() {
    let hist = simple_filled_float_mean_hist();
    let binvalue = hist.value(&0.0).unwrap();
    assert_eq!(binvalue.num_samples(), 3);
}
#[test]
fn test_mean_value_stddev_samples() {
    let hist = simple_filled_float_mean_hist();
    let binvalue = hist.value(&0.0).unwrap();
    assert_float_eq(binvalue.variance_of_samples(), 2.0 / 3.0);
    assert_float_eq(
        binvalue.standard_deviation_of_samples(),
        (2.0f64 / 3.0).sqrt(),
    );
}

#[test]
fn test_mean_value_stderr() {
    let hist = simple_filled_float_mean_hist();
    let binvalue = hist.value(&0.0).unwrap();
    assert_float_eq(binvalue.variance_of_mean(), (2.0 / 3.0) / 3.0);
    assert_float_eq(
        binvalue.standard_error_of_mean(),
        ((2.0f64 / 3.0) / 3.0).sqrt(),
    );
}

fn simple_filled_int_mean_hist() -> VecHistogram<(Uniform,), Mean<i32>> {
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0); Mean<i32>);
    hist.fill_with(&0.0, 1);
    hist.fill_with(&0.0, 2);
    hist.fill_with(&0.0, 3);
    hist
}
#[test]
fn test_mean_value_mean_int() {
    let hist = simple_filled_float_mean_hist();
    assert_float_eq(hist.value(&0.0).unwrap().get(), 2.0)
}
#[test]
fn test_mean_int_value_stddev_samples() {
    let hist = simple_filled_int_mean_hist();
    let binvalue = hist.value(&0.0).unwrap();
    assert_float_eq(binvalue.variance_of_samples(), 2.0 / 3.0);
    assert_float_eq(
        binvalue.standard_deviation_of_samples() as f64,
        (2.0f64 / 3.0).sqrt(),
    );
}

#[test]
fn test_mean_int_value_stderr() {
    let hist = simple_filled_int_mean_hist();
    let binvalue = hist.value(&0.0).unwrap();
    assert_float_eq(binvalue.variance_of_mean(), (2.0 / 3.0) / 3.0);
    assert_float_eq(
        binvalue.standard_error_of_mean(),
        ((2.0f64 / 3.0) / 3.0).sqrt(),
    );
}
