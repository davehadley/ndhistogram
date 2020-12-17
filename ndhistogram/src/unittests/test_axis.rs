use crate::axis::Axis;
use crate::axis::Uniform;

#[test]
fn test_uniform_numbins() {
    let nbins = 100;
    let ax = Uniform {
        num: nbins,
        low: 0.0,
        high: 1.0,
    };
    assert_eq!(ax.numbins(), nbins)
}
