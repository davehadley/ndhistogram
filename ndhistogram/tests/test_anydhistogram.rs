use ndhistogram::{axis::Uniform, AxesAnyD};

#[test]
fn test_anyd_1d() {
    let ax = Uniform::new(100, -5.0, 5.0);
    let axvec = vec![Box::new(ax.clone())];
    let actual = AxesAnyD::new(axvec);
    let expected = (ax.clone(),);
}
