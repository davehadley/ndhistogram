use ndhistogram::{
    axis::{Category, CategoryNoFlow, Uniform, UniformNoFlow, Variable, VariableNoFlow},
    ndhistogram,
    value::{Mean, Sum, WeightedMean, WeightedSum},
    Hist1D, HistND, Histogram,
};
use rand::{prelude::StdRng, Rng, SeedableRng};

macro_rules! test_serialize_empty_impl {
    ($fnname:ident; $Type:ty; $construct:expr) => {
        #[test]
        fn $fnname() {
            let hist: $Type = $construct;
            let serialized = serde_json::to_string(&hist).unwrap();
            let deserialized: $Type = serde_json::from_str(&serialized).unwrap();
            assert_eq!(hist, deserialized)
        }
    };
}

test_serialize_empty_impl! {
    test_serialized_vec_histogram_1d_f64;
    Hist1D<Uniform>;
    ndhistogram!(Uniform::new(10, -5.0, 5.0); f64)
}

test_serialize_empty_impl! {
    test_serialized_vec_histogram_6d_f64;
    HistND<(Uniform, Variable<i32>, Category<&str>, UniformNoFlow, VariableNoFlow<i32>, CategoryNoFlow<&str>)>;
    ndhistogram!(
        Uniform::new(10, -5.0, 5.0),
        Variable::new(vec![0, 2, 4, 8, 16]),
        Category::new(vec!["A", "B", "C"]),
        UniformNoFlow::new(10, -5.0, 5.0),
        VariableNoFlow::new(vec![0, 2, 4, 8, 16]),
        CategoryNoFlow::new(vec!["A", "B", "C"]),
        ; f64
    )
}

macro_rules! test_serialize_filled_value_impl {
    ($fnname:ident; $Type:ty; $hist:ident; $rng:ident; $fillexpr:expr) => {
        #[test]
        fn $fnname() {
            let mut $hist: Hist1D<Uniform, $Type> = ndhistogram!(Uniform::new(10, -5.0, 5.0); $Type);
            let mut $rng = StdRng::seed_from_u64(123);
            (0..1000).for_each(|_| $fillexpr);
            let serialized = serde_json::to_string(&$hist).unwrap();
            let deserialized: Hist1D<Uniform, $Type> = serde_json::from_str(&serialized).unwrap();
            assert_eq!($hist, deserialized)
        }
    }
}

test_serialize_filled_value_impl! {
    test_serialized_vec_histogram_1d_sum;
    Sum;
    hist;
    rng; hist.fill(&rng.gen_range(-6.0..6.0))
}

test_serialize_filled_value_impl! {
    test_serialized_vec_histogram_1d_weightedsum;
    WeightedSum;
    hist;
    rng;
    hist.fill_with(&rng.gen_range(-6.0..6.0), rng.gen_range(0.0..10.0))
}

test_serialize_filled_value_impl! {
    test_serialized_vec_histogram_1d_mean;
    Mean<i64, i64>;
    hist;
    rng;
    hist.fill_with(&rng.gen_range(-6.0..6.0), rng.gen_range(0..10))
}

test_serialize_filled_value_impl! {
    test_serialized_vec_histogram_1d_weightedmean;
    WeightedMean<i64, i64, i64>;
    hist;
    rng;
    hist.fill_with_weighted(&rng.gen_range(-6.0..6.0), rng.gen_range(0..10), rng.gen_range(0..10))
}
