use crate::{axis::Uniform, histogram::Histogram};

#[test]
#[allow(clippy::float_cmp)]
fn test_histogram_uniform_2d_unweighted_fill_once() {
    let mut hist = ndhistogram!(Uniform::new(5, 0.0, 0.5), Uniform::new(5, 0.5, 1.0));
    hist.fill((0.1, 0.6));
    let actual = *hist.value((0.1, 0.6)).unwrap();
    let expected = 1.0;
    assert_eq!(expected, actual);
}

#[test]
#[allow(clippy::float_cmp)]
fn test_histogram_uniform_2d_unfilled_is_empty() {
    let hist = ndhistogram!(Uniform::new(5, 0.0, 0.5), Uniform::new(5, 0.5, 1.0));
    let actual: Vec<f64> = hist.iter_values().copied().collect();
    let expected = vec![0.0; 7 * 7];
    assert_eq!(expected, actual);
}
