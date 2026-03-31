use std::collections::HashMap;

use ndhistogram::{axis::Uniform, ndhistogram, sparsehistogram, Histogram};

#[test]
fn test_vec_histogram_as_vec() {
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0).unwrap());
    hist.fill(&0.5);
    let actual = hist.as_vec();
    let expected: &Vec<f64> = &vec![0.0, 1.0, 0.0];
    assert_eq!(expected, actual);
}

#[test]
fn test_vec_histogram_as_slice() {
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0).unwrap());
    hist.fill(&0.5);
    let actual = hist.as_slice();
    let expected: &[f64] = &[0.0, 1.0, 0.0];
    assert_eq!(expected, actual);
}

#[test]
fn test_vec_histogram_into_vec() {
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0).unwrap());
    hist.fill(&0.5);
    let actual: Vec<f64> = hist.into();
    let expected: Vec<f64> = vec![0.0, 1.0, 0.0];
    assert_eq!(expected, actual);
}

#[test]
fn test_hashhistogram_as_map() {
    let mut hist = sparsehistogram!(Uniform::new(1, 0.0, 1.0).unwrap());
    hist.fill(&0.5);
    let actual = hist.as_map();
    let expected: &HashMap<usize, f64> = &HashMap::from([(1, 1.0)]);
    assert_eq!(expected, actual);
}

#[test]
fn test_hashhistogram_into_map() {
    let mut hist = sparsehistogram!(Uniform::new(1, 0.0, 1.0).unwrap());
    hist.fill(&0.5);
    let actual: HashMap<usize, f64> = hist.into();
    let expected: HashMap<usize, f64> = HashMap::from([(1, 1.0)]);
    assert_eq!(expected, actual);
}
