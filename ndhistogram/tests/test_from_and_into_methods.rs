use ndhistogram::{axis::Uniform, ndhistogram, Histogram};

#[test]
fn test_vec_histogram_as_vec() {
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0).unwrap());
    hist.fill(&0.5);
    let actual = hist.as_vec();
    let expected: &Vec<f64> = &vec![0.0, 1.0, 0.0];
    assert_eq!(expected, actual);
}
