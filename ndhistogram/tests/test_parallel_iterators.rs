use ndhistogram::{axis::Uniform, ndhistogram, Histogram};
use rayon::prelude::*;

#[test]
fn test_vec_histogram_par_values() {
    let mut hist = ndhistogram!(Uniform::new(1000, 0.0, 100.0));
    (0..100).for_each(|it| hist.fill_with(&(it as f64), it as f64));
    let par_values: Vec<_> = hist.par_values().collect();
    let values: Vec<_> = hist.values().collect();
    // rayon preserves original order on collect
    // https://github.com/rayon-rs/rayon/issues/551
    assert_eq!(values, par_values);
}

#[test]
fn test_vec_histogram_par_values_mut() {
    let mut hist = ndhistogram!(Uniform::new(1000, 0.0, 100.0));
    (0..100).for_each(|it| hist.fill_with(&(it as f64), it as f64));
    let double_original_values: Vec<f64> = hist.values().map(|it| it * 2.0).collect();
    hist.par_values_mut().for_each(|it| *it *= 2.0);
    let new_values: Vec<f64> = hist.values().map(|it| *it).collect();
    assert_eq!(double_original_values, new_values);
}

#[test]
fn test_vec_histogram_par_iter() {
    let mut hist = ndhistogram!(Uniform::new(1000, 0.0, 100.0));
    (0..100).for_each(|it| hist.fill_with(&(it as f64), it as f64));
    let par_values: Vec<_> = hist.par_iter().map(|it| it.value).collect();
    let values: Vec<_> = hist.values().collect();
    assert_eq!(values, par_values);
}
