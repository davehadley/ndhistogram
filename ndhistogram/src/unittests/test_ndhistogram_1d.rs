use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Normal};

use crate::{
    axis::{Axis, Uniform},
    histogram::{ArrayHistogram, Fill, FillWeight, Histogram, Item},
};

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
    let actual: Vec<f64> = hist.values().copied().collect();
    let expected = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    assert_eq!(expected, actual);
}

#[test]
#[allow(clippy::float_cmp)]
fn test_histogram_uniform_1d_weighted_fill_once() {
    let mut hist = ndhistogram!(Uniform::new(5, 0.0, 0.5));
    hist.fill_weight(&0.1, 2.0);
    let actual = *hist.value(&0.1).unwrap();
    let expected = 2.0;
    assert_eq!(expected, actual);
}

#[test]
#[allow(clippy::float_cmp)]
fn test_histogram_uniform_1d_unweighted_fill_bin_edges() {
    let mut hist = ndhistogram!(Uniform::new(2, 0.0, 2.0));
    hist.fill(&-1.0);
    hist.fill_weight(&0.0, 20.0);
    hist.fill_weight(&0.5, 300.0);
    hist.fill_weight(&1.0, 4000.0);
    hist.fill_weight(&1.5, 50000.0);
    hist.fill_weight(&2.0, 600000.0);
    hist.fill_weight(&2.5, 7000000.0);
    let actual: Vec<f64> = hist.values().copied().collect();
    assert_eq!(actual, vec![1.0, 320.0, 54000.0, 7600000.0]);
}

#[test]
fn test_histogram_uniform_1d_get_axes() {
    let ax = Uniform::new(2, 0.0, 2.0);
    let hist = ndhistogram!(ax.clone());
    let axtuple = hist.axes();
    assert_eq!(ax, axtuple.0);
}

#[test]
fn test_histogram_uniform_1d_value_at_index() {
    let mut hist = ndhistogram!(Uniform::new(2, 0.0, 2.0));
    hist.fill(&0.0);
    assert_eq!(hist.value_at_index(0), Some(&0.0));
    assert_eq!(hist.value_at_index(1), Some(&1.0));
    assert_eq!(hist.value_at_index(2), Some(&0.0));
    assert_eq!(hist.value_at_index(3), Some(&0.0));
    assert_eq!(hist.value_at_index(4), None);
}

#[test]
fn test_histogram_uniform_1d_value_at_coordinate() {
    let mut hist = ndhistogram!(Uniform::new(2, 0.0, 2.0));
    hist.fill(&0.0);
    assert_eq!(hist.value(&0.0), Some(&1.0));
    assert_eq!(hist.value(&1.0), Some(&0.0));
    assert_eq!(hist.value(&-1.0), Some(&0.0));
    assert_eq!(hist.value(&2.0), Some(&0.0));
}

fn make_normal_histogram() -> impl Histogram<(Uniform,), f64> {
    let mut hist = ndhistogram!(Uniform::new(5, 0.0, 5.0));
    Normal::new(0.0, 5.0)
        .unwrap()
        .sample_iter(thread_rng())
        .take(100)
        .for_each(|it| hist.fill(&it));
    hist
}

#[test]
fn test_histogram_value_iterator() {
    let hist = make_normal_histogram();
    let actual: Vec<_> = hist.values().collect();
    let expected: Vec<_> = (0..7).map(|it| hist.value_at_index(it).unwrap()).collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_histogram_item_iterator() {
    let hist = make_normal_histogram();
    let actual: Vec<_> = hist.iter().collect();
    let expected: Vec<_> = (0..7)
        .map(|it| {
            Item::new(
                it,
                hist.axes().bin(it).unwrap(),
                hist.value_at_index(it).unwrap(),
            )
        })
        .collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_histogram_uniform_1d_value_at_index_mut() {
    let mut hist = ndhistogram!(Uniform::new(2, 0.0, 2.0));
    *(hist.value_at_index_mut(1).unwrap()) = 2.0;
    assert_eq!(hist.value_at_index(0), Some(&0.0));
    assert_eq!(hist.value_at_index(1), Some(&2.0));
    assert_eq!(hist.value_at_index(2), Some(&0.0));
    assert_eq!(hist.value_at_index(3), Some(&0.0));
    assert_eq!(hist.value_at_index(4), None);
}

#[test]
fn test_histogram_uniform_1d_value_mut() {
    let mut hist = ndhistogram!(Uniform::new(2, 0.0, 2.0));
    *(hist.value_mut(&0.0).unwrap()) = 2.0;
    assert_eq!(hist.value_at_index(0), Some(&0.0));
    assert_eq!(hist.value_at_index(1), Some(&2.0));
    assert_eq!(hist.value_at_index(2), Some(&0.0));
    assert_eq!(hist.value_at_index(3), Some(&0.0));
    assert_eq!(hist.value_at_index(4), None);
}

#[test]
fn test_histogram_uniform_1d_iter_values_mut() {
    let mut hist = ndhistogram!(Uniform::new(2, 0.0, 2.0));
    hist.values_mut()
        .enumerate()
        .for_each(|(index, value)| *value = index as f64);
    assert_eq!(hist.value_at_index(0), Some(&0.0));
    assert_eq!(hist.value_at_index(1), Some(&1.0));
    assert_eq!(hist.value_at_index(2), Some(&2.0));
    assert_eq!(hist.value_at_index(3), Some(&3.0));
    assert_eq!(hist.value_at_index(4), None);
}

#[test]
fn test_histogram_uniform_1d_iter_mut() {
    let mut hist = ndhistogram!(Uniform::new(2, 0.0, 2.0));
    (&mut hist)
        .into_iter()
        .for_each(|item| *item.value = item.index as f64);
    assert_eq!(hist.value_at_index(0), Some(&0.0));
    assert_eq!(hist.value_at_index(1), Some(&1.0));
    assert_eq!(hist.value_at_index(2), Some(&2.0));
    assert_eq!(hist.value_at_index(3), Some(&3.0));
    assert_eq!(hist.value_at_index(4), None);
}
