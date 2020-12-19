use crate::axis::Uniform;
use crate::{binvalues::VecBinValues, histogram::Histogram};

#[cfg(test)] // Apply cfg to file
#[test]
fn test_builds() {
    assert!(true);
}

#[test]
fn test_histogram1d_fill() {
    let mut hist = Histogram::new((Uniform::new(10, 0.0, 1.0),), VecBinValues::<f64>::new(10));
    hist.fill(&0.1, 1.0);
}
