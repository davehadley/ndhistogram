use crate::axes::{Axes, Axes1D};
use crate::axis::Uniform;

#[cfg(test)]
#[test]
fn test_axes1d_uniform() {
    let nbins = 5;
    let ax = Axes1D::new(Uniform::new(nbins, 0.0, 5.0));
    let actual: Vec<usize> = (-2..7).map(|it| ax.index(&(it as f64))).collect();
    let expected = vec![0, 0, 1, 2, 3, 4, 5, 6, 6];
    assert_eq!(expected, actual)
}
