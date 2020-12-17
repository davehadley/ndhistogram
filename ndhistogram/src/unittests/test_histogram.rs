use super::super::Histogram1D;
use super::super::Uniform;

#[test]

fn test_histogram_1d_uniform() {
    let h: Histogram1D<Uniform> = Histogram1D::new(Uniform::new(2, 0.0, 1.0));
}
