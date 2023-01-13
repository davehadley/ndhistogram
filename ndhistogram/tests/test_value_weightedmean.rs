use ndhistogram::{axis::Uniform, ndhistogram, value::WeightedMean, Hist1D, Histogram};

fn assert_float_eq(left: f64, right: f64) {
    assert!(
        (left - right).abs() < (1e-6 * (left + right)),
        "left={} does not equal right={}",
        left,
        right
    )
}

fn simple_filled_float_weightedmean_hist_with_unit_weights(
) -> Hist1D<Uniform, WeightedMean<f64, f64>> {
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0); WeightedMean<f64, f64>);
    hist.fill_with_weighted(&0.0, 1.0, 1.0);
    hist.fill_with_weighted(&0.0, 2.0, 1.0);
    hist.fill_with_weighted(&0.0, 3.0, 1.0);
    hist
}

#[test]
fn test_weightedmean_value_weightedmean() {
    let hist = simple_filled_float_weightedmean_hist_with_unit_weights();
    assert_float_eq(hist.value(&0.0).unwrap().get(), 2.0)
}

#[test]
fn test_weightedmean_value_numsamples() {
    let hist = simple_filled_float_weightedmean_hist_with_unit_weights();
    let binvalue = hist.value(&0.0).unwrap();
    assert_eq!(binvalue.num_samples(), 3);
}
#[test]
fn test_weightedmean_value_stddev_samples() {
    let hist = simple_filled_float_weightedmean_hist_with_unit_weights();
    let binvalue = hist.value(&0.0).unwrap();
    assert_float_eq(binvalue.variance_of_samples(), 2.0 / 3.0);
    assert_float_eq(
        binvalue.standard_deviation_of_samples(),
        (2.0f64 / 3.0).sqrt(),
    );
}

#[test]
fn test_weightedmean_value_stderr() {
    let hist = simple_filled_float_weightedmean_hist_with_unit_weights();
    let binvalue = hist.value(&0.0).unwrap();
    assert_float_eq(binvalue.variance_of_mean(), (2.0 / 3.0) / 3.0);
    assert_float_eq(
        binvalue.standard_error_of_mean(),
        ((2.0f64 / 3.0) / 3.0).sqrt(),
    );
}

fn simple_filled_int_weightedmean_hist_with_unit_weights() -> Hist1D<Uniform, WeightedMean<i32, i32>>
{
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0); WeightedMean<i32, i32>);
    hist.fill_with_weighted(&0.0, 1, 1);
    hist.fill_with_weighted(&0.0, 2, 1);
    hist.fill_with_weighted(&0.0, 3, 1);
    hist
}
#[test]
fn test_weightedmean_value_weightedmean_int() {
    let hist = simple_filled_int_weightedmean_hist_with_unit_weights();
    assert_float_eq(hist.value(&0.0).unwrap().get(), 2.0)
}
#[test]
fn test_weightedmean_int_value_stddev_samples() {
    let hist = simple_filled_int_weightedmean_hist_with_unit_weights();
    let binvalue = hist.value(&0.0).unwrap();
    assert_float_eq(binvalue.variance_of_samples(), 2.0 / 3.0);
    assert_float_eq(
        binvalue.standard_deviation_of_samples(),
        (2.0f64 / 3.0).sqrt(),
    );
}

#[test]
fn test_weightedmean_int_value_stderr() {
    let hist = simple_filled_int_weightedmean_hist_with_unit_weights();
    let binvalue = hist.value(&0.0).unwrap();
    assert_float_eq(binvalue.variance_of_mean(), (2.0 / 3.0) / 3.0);
    assert_float_eq(
        binvalue.standard_error_of_mean(),
        ((2.0f64 / 3.0) / 3.0).sqrt(),
    );
}

fn simple_filled_float_weightedmean_hist_with_weights(
) -> Hist1D<Uniform, WeightedMean<f64, f64, f64, u32>> {
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0); WeightedMean<f64, f64, f64, u32>);
    //let mut hist = VecHistogram::<_, WeightedMean<f64, f64, f64, u32>>::new((Uniform::new(1, 0.0, 1.0),));
    hist.fill_with_weighted(&0.0, 1.0, 1.0);
    hist.fill_with_weighted(&0.0, 2.0, 2.0);
    hist.fill_with_weighted(&0.0, 3.0, 3.0);
    hist
}

#[test]
fn test_weightedmean_value_weightedmean_with_weights() {
    let hist = simple_filled_float_weightedmean_hist_with_weights();
    assert_float_eq(hist.value(&0.0).unwrap().get(), 14. / 6.0)
}

#[test]
fn test_weightedmean_value_numsamples_with_weights() {
    let hist = simple_filled_float_weightedmean_hist_with_weights();
    let binvalue = hist.value(&0.0).unwrap();
    assert_eq!(binvalue.num_samples(), 3);
}
#[test]
fn test_weightedmean_value_stddev_samples_with_weights() {
    let hist = simple_filled_float_weightedmean_hist_with_weights();
    let binvalue = hist.value(&0.0).unwrap();
    let expected = 0.5555555555555555;
    assert_float_eq(binvalue.variance_of_samples(), expected);
    assert_float_eq(binvalue.standard_deviation_of_samples(), expected.sqrt());
}

#[test]
fn test_weightedmean_value_stderr_with_weights() {
    let hist = simple_filled_float_weightedmean_hist_with_weights();
    let binvalue = hist.value(&0.0).unwrap();
    let expected = 0.21604938271604893;
    assert_float_eq(binvalue.variance_of_mean(), expected);
    assert_float_eq(binvalue.standard_error_of_mean(), expected.sqrt());
}
