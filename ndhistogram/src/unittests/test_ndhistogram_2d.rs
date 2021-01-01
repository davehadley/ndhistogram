use std::collections::HashMap;

use crate::{
    axis::{Axis, Uniform},
    histogram::{Fill, FillWeight, Histogram},
};

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
    let actual: Vec<f64> = hist.values().copied().collect();
    let expected = vec![0.0; 7 * 7];
    assert_eq!(expected, actual);
}

#[test]
#[allow(clippy::float_cmp)]
fn test_histogram_uniform_2d_weighted_fill_once() {
    let mut hist = ndhistogram!(Uniform::new(5, 0.0, 0.5), Uniform::new(5, 0.5, 1.0));
    hist.fill_weight((0.1, 0.6), 2.0);
    let actual = *hist.value((0.1, 0.6)).unwrap();
    let expected = 2.0;
    assert_eq!(expected, actual);
}

#[test]
#[allow(clippy::float_cmp)]
fn test_histogram_uniform_2d_unweighted_fill_bin_edges() {
    let mut hist = ndhistogram!(Uniform::new(2, 0.0, 2.0), Uniform::new(2, 0.0, 2.0));
    hist.fill((-1.0, -1.0));
    hist.fill_weight((0.0, 0.0), 20.0);
    hist.fill_weight((1.0, 0.0), 300.0);
    hist.fill_weight((0.0, 1.0), 4000.0);
    hist.fill_weight((1.0, 1.0), 50000.0);
    let actual: Vec<((usize, usize), f64)> = hist
        .axes()
        .bins()
        //.flatten()
        .map(|bin| {
            (
                (
                    hist.axes().0.index(bin.0.start().unwrap_or(-1.0)),
                    hist.axes().1.index(bin.1.start().unwrap_or(-1.0)),
                ),
                *hist
                    .value((bin.0.start().unwrap_or(-1.0), bin.1.start().unwrap_or(-1.0)))
                    .unwrap(),
            )
        })
        .collect();
    let expected = vec![
        ((0, 0), 1.0),
        ((1, 0), 0.0),
        ((2, 0), 0.0),
        ((3, 0), 0.0),
        ((0, 1), 0.0),
        ((1, 1), 20.0),
        ((2, 1), 300.0),
        ((3, 1), 0.0),
        ((0, 2), 0.0),
        ((1, 2), 4000.0),
        ((2, 2), 50000.0),
        ((3, 2), 0.0),
        ((0, 3), 0.0),
        ((1, 3), 0.0),
        ((2, 3), 0.0),
        ((3, 3), 0.0),
    ];
    assert_eq!(expected, actual);
}
