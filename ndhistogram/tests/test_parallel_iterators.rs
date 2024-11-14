#[cfg(feature = "rayon")]
mod rayon_tests {

    use ndhistogram::{
        axis::Uniform, ndhistogram, sparsehistogram, Hist1D, Histogram, SparseHist1D,
    };

    use rayon::prelude::*;

    fn example_filled_vec_histogram() -> Hist1D<Uniform> {
        let mut hist = ndhistogram!(Uniform::new(100, 0.0, 100.0).unwrap());
        (-1..=101).for_each(|it| hist.fill_with(&(it as f64), it as f64));
        hist
    }

    #[test]

    fn test_vec_histogram_par_values() {
        let hist = example_filled_vec_histogram();
        let par_values: Vec<_> = hist.par_values().collect();
        let values: Vec<_> = hist.values().collect();
        // rayon preserves original order on collect
        // https://github.com/rayon-rs/rayon/issues/551
        assert_eq!(values, par_values);
    }

    #[test]

    fn test_vec_histogram_par_values_mut() {
        let mut hist = example_filled_vec_histogram();
        let double_original_values: Vec<f64> = hist.values().map(|it| it * 2.0).collect();
        hist.par_values_mut().for_each(|it| *it *= 2.0);
        let new_values: Vec<f64> = hist.values().copied().collect();
        assert_eq!(double_original_values, new_values);
    }

    #[test]

    fn test_vec_histogram_par_iter() {
        let hist = example_filled_vec_histogram();
        let par_values: Vec<_> = hist.par_iter().map(|it| it.value).collect();
        let values: Vec<_> = hist.values().collect();
        assert_eq!(values, par_values);
    }

    #[test]

    fn test_vec_histogram_par_iter_mut() {
        let mut hist = example_filled_vec_histogram();
        let double_original_values: Vec<f64> = hist.values().map(|it| it * 2.0).collect();
        hist.par_iter_mut().for_each(|it| *it.value *= 2.0);
        let new_values: Vec<f64> = hist.values().copied().collect();
        assert_eq!(double_original_values, new_values);
    }

    fn example_filled_hash_histogram() -> SparseHist1D<Uniform, i64> {
        let mut hist = sparsehistogram!(Uniform::new(100, 0.0, 100.0).unwrap(); i64);
        (-1..=101).for_each(|it| hist.fill_with(&(it as f64), it));
        hist
    }

    #[test]

    fn test_hash_histogram_par_values() {
        let hist = example_filled_hash_histogram();
        let par_values: Vec<_> = hist.par_values().collect();
        let values: Vec<_> = hist.values().collect();
        // rayon preserves original order on collect
        // https://github.com/rayon-rs/rayon/issues/551
        assert_eq!(values, par_values);
    }

    #[test]

    fn test_hash_histogram_par_values_mut() {
        let mut hist = example_filled_hash_histogram();
        let double_original_values: Vec<_> = hist.values().map(|it| it * 2).collect();
        hist.par_values_mut().for_each(|it| *it *= 2);
        let new_values: Vec<_> = hist.values().copied().collect();
        assert_eq!(double_original_values, new_values);
    }

    #[test]

    fn test_hash_histogram_par_iter() {
        let hist = example_filled_hash_histogram();
        let par_values: Vec<_> = hist.par_iter().map(|it| it.value).collect();
        let values: Vec<_> = hist.values().collect();
        assert_eq!(values, par_values);
    }

    #[test]

    fn test_hash_histogram_par_iter_mut() {
        let mut hist = example_filled_hash_histogram();
        let double_original_values: Vec<_> = hist.values().map(|it| it * 2).collect();
        hist.par_iter_mut().for_each(|it| *it.value *= 2);
        let new_values: Vec<_> = hist.values().copied().collect();
        assert_eq!(double_original_values, new_values);
    }
}
