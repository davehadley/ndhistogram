extern crate ndhistogram;
use ndhistogram::axis::Uniform;
use ndhistogram::{histogram::Histogram, ndhistogram};

fn main() {
    let mut hist = ndhistogram!(Uniform::new(10, 0.0, 1.0), Uniform::new(10, 1.0, 2.0));

    // fill a single number
    hist.fill(&(0.0, 1.0));
}
