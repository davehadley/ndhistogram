extern crate ndhistogram;
use ndhistogram::ndhistogram;
use ndhistogram::{axis::Uniform, histogram::Fill};

fn main() {
    let mut hist = ndhistogram!(Uniform::new(10, 0.0, 1.0), Uniform::new(10, 1.0, 2.0));

    // fill a single number
    hist.fill((0.0, 1.0));
}

#[test]
#[allow(clippy::float_cmp)]
fn test_histogram_uniform_1d_unweighted_fill_bin_edges() {
    let mut hist = ndhistogram!(Uniform::new(2, 0.0, 2.0));
    hist.fill(-1.0);
    hist.fill_weight(0.0, 20.0);
    hist.fill_weight(0.5, 300.0);
    hist.fill_weight(1.0, 4000.0);
    hist.fill_weight(1.5, 50000.0);
    hist.fill_weight(2.0, 600000.0);
    hist.fill_weight(2.5, 7000000.0);
    let actual: Vec<f64> = hist.iter_values().copied().collect();
    assert_eq!(actual, vec![1.0, 320.0, 54000.0, 7600000.0]);
}
