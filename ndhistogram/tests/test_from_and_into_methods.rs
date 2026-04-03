use std::{
    collections::HashMap,
    hash::{BuildHasher, RandomState},
};

use ndhistogram::{
    axis::Uniform, error::AxisError, ndhistogram, sparsehistogram, AxesTuple, DefaultHasher, Error,
    HashHistogram, Histogram, VecHistogram,
};

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
fn test_vec_histogram_from_vec() {
    let axes: AxesTuple<_> = (Uniform::new(1, 0.0, 1.0).unwrap(),).into();
    let hist = VecHistogram::<_, _>::from_vec(axes, vec![0.0, 1.0, 0.0]);
    assert!(hist.is_ok());
}

#[test]
fn test_vec_histogram_from_vec_with_invalid_bin_numbers() {
    let axes: AxesTuple<_> = (Uniform::new(1, 0.0, 1.0).unwrap(),).into();
    let result = VecHistogram::<_, _>::from_vec(axes, vec![0.0]);
    assert_eq!(
        result,
        Err(Error::AxisError(AxisError::InvalidNumberOfBins))
    );
}

#[test]
fn test_hashhistogram_as_map() {
    let mut hist = sparsehistogram!(Uniform::new(1, 0.0, 1.0).unwrap());
    hist.fill(&0.5);
    let actual = hist.as_map();
    let expected: &HashMap<usize, f64, DefaultHasher> = &HashMap::from_iter([(1, 1.0)]);
    assert_eq!(expected, actual);
}

#[test]
fn test_hashhistogram_into_map() {
    let mut hist = sparsehistogram!(Uniform::new(1, 0.0, 1.0).unwrap());
    hist.fill(&0.5);
    let actual: HashMap<usize, f64, DefaultHasher> = hist.into();
    let expected: HashMap<usize, f64, DefaultHasher> = HashMap::from_iter([(1, 1.0)]);
    assert_eq!(expected, actual);
}

#[test]
fn test_hash_histogram_from_hashmap() {
    let axes: AxesTuple<_> = (Uniform::new(1, 0.0, 1.0).unwrap(),).into();
    let hist = HashHistogram::<_, _>::from_map(axes, HashMap::from_iter([(1, 1.0)]));
    assert!(hist.is_ok());
}

#[test]
fn test_hash_histogram_from_hashmap_with_invalid_bin_numbers() {
    let axes: AxesTuple<_> = (Uniform::new(1, 0.0, 1.0).unwrap(),).into();
    let result = HashHistogram::<_, _>::from_map(axes, HashMap::from_iter([(3, 1.0)]));
    assert_eq!(
        result,
        Err(Error::AxisError(AxisError::InvalidNumberOfBins))
    );
}

#[test]
fn test_hash_histogram_with_user_supplied_hasher() {
    let axes: AxesTuple<_> = (Uniform::new(1, 0.0, 1.0).unwrap(),).into();

    #[derive(Default)]
    struct CustomHasher;

    impl BuildHasher for CustomHasher {
        type Hasher = std::hash::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::DefaultHasher::default()
        }
    }

    let mut hist = HashHistogram::with_hasher(axes, CustomHasher);
    hist.fill(&0.5);
    let actual: HashMap<usize, f64, CustomHasher> = hist.into();
    let expected: HashMap<usize, f64, CustomHasher> = HashMap::from_iter([(1, 1.0)]);
    assert_eq!(actual, expected);
}
