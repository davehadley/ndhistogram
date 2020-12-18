use crate::axis::Uniform;
use crate::{axes::Axes1D, binvalues::VecBinValues, histogram::Histogram};

#[cfg(test)] // Apply cfg to file
#[test]
fn test_builds() {
    assert!(true);
}

#[test]
fn test_histogram1d_fill() {
    let mut hist = Histogram::new(
        Axes1D::new(Uniform::new(10, 0.0, 1.0)),
        VecBinValues::<f64>::new(10),
    );
    hist.fill(&2.0, 1.0);
}
