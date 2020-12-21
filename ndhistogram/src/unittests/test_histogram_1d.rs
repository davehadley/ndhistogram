use crate::axis::Uniform;

#[test]
fn test_histogram_1d_uniform_constructor() {
    let hist = ndhistogram!(Uniform::new(5, 0.0, 1.0));
}
