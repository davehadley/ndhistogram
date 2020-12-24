use crate::{axis::Uniform, histogram::Histogram};

#[test]
#[allow(clippy::float_cmp)]
fn test_histogram_uniform_1d_unweighted_fill_once() {
    let mut hist = ndhistogram!(Uniform::new(5, 0.0, 0.5));
    hist.fill(&0.1);
    let actual = *hist.value(&0.1).unwrap();
    let expected = 1.0;
    assert_eq!(expected, actual);
}

#[test]
#[allow(clippy::float_cmp)]
fn test_histogram_uniform_1d_unfilled_is_empty() {
    let hist = ndhistogram!(Uniform::new(5, 0.0, 0.5));
    let actual: Vec<f64> = hist.iter_values().copied().collect();
    let expected = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    assert_eq!(expected, actual);
}
