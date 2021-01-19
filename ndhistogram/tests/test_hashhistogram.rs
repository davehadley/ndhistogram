use ndhistogram::{axis::Uniform, sparsehistogram, Histogram};

#[test]
fn test_hashhistogram_fill() {
    let mut hist = sparsehistogram!(Uniform::new(10, 0.0, 10.0); i64);
    hist.fill(&0.5);
    hist.fill_with(&0.5, 2);
    assert_eq!(hist.value(&0.5), Some(&3))
}
