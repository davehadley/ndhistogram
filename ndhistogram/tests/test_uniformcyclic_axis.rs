use ndhistogram::axis::{Axis, BinInterval, UniformCyclic};

use ndhistogram::{ndhistogram, Histogram};

use rstest::rstest;

#[rstest(/**/bin_no,      expected_interval,
         case(0    , Some(BinInterval::new(0.00, 0.25))),
         case(1    , Some(BinInterval::new(0.25, 0.50))),
         case(2    , Some(BinInterval::new(0.50, 0.75))),
         case(3    , Some(BinInterval::new(0.75, 1.00))),
)]
fn bin(bin_no: usize, expected_interval: Option<BinInterval<f32>>) {
    let axis = UniformCyclic::new(4, 0.0, 1.0).unwrap();
    assert_eq!(axis.bin(bin_no), expected_interval);
}

#[rstest(coordinate, expected_index,
         case(  0.0 , Some(0)),
         case(  0.09, Some(0)),
         case(  0.1 , Some(1)),
         case(  0.19, Some(1)),
         case(  0.2 , Some(2)),
         case( 10.0 , Some(0)),
         case( 20.33, Some(3)),
         case( 50.99, Some(9)),
         case( -0.1 , Some(9)),
         case( -0.19, Some(8)),
         case( -0.2 , Some(8)),
         case( -0.9 , Some(1)),
         case( -0.95, Some(0)),
         case(-10.0 , Some(0)),
         case(-10.05, Some(9)),
         case(-10.1 , Some(8)),
)]
fn float_index(coordinate: f32, expected_index: Option<usize>) {
    let axis = UniformCyclic::new(10, 0.0, 1.0).unwrap();
    assert_eq!(axis.index(&coordinate), expected_index);
}

#[rstest(/**/ nbins, low, step, coordinate, expected_index,
         case(  2  ,  0 ,  10 ,      0    ,     Some(0)  ),
         case(  2  ,  0 ,  10 ,      5    ,     Some(0)  ),
         case(  2  ,  0 ,  10 ,     15    ,     Some(1)  ),
         case(  2  ,  0 ,  10 ,     20    ,     Some(0)  ),
         case(  2  ,  0 ,  10 ,     29    ,     Some(0)  ),
         case(  2  ,  0 ,  10 ,     30    ,     Some(1)  ),
         case(  2  ,  0 ,  10 ,     -1    ,     Some(1)  ),
         case(  2  ,  0 ,  10 ,    -10    ,     Some(1)  ),
         case(  2  ,  0 ,  10 ,    -11    ,     Some(0)  ),
         case( 10  , 10 ,   9 ,     10    ,     Some(0)  ),
         case( 10  , 10 ,   9 ,     18    ,     Some(0)  ),
         case( 10  , 10 ,   9 ,     19    ,     Some(1)  ),
         case( 10  , 10 ,   9 ,     27    ,     Some(1)  ),
         case( 10  , 10 ,   9 ,     28    ,     Some(2)  ),
         case( 10  , 10 ,   9 ,     99    ,     Some(9)  ),
         case( 10  , 10 ,   9 ,    100    ,     Some(0)  ),
)]
fn integer_index(
    nbins: usize,
    low: i32,
    step: i32,
    coordinate: i32,
    expected_index: Option<usize>,
) {
    let axis = UniformCyclic::with_step_size(nbins, low, step).unwrap();
    assert_eq!(axis.index(&coordinate), expected_index);
}

#[test]
fn indices() {
    let n = 7;
    let axis = UniformCyclic::new(n, 23.4, 97.3).unwrap();
    let indices = axis.indices().collect::<Vec<_>>();
    assert_eq!(indices, (0..n).collect::<Vec<_>>());
}

#[rstest]
fn wrap_float_fill() {
    let mut hist = ndhistogram!(UniformCyclic::new(4, 0.0, 360.0).unwrap(); u8);
    hist.fill(&45.0);
    hist.fill(&(45.0 + 360.0));
    hist.fill(&(45.0 - 360.0));
    assert_eq!(hist.value(&45.0), Some(&3));
    assert_eq!(hist.value_at_index(0), Some(&3));
}

#[rstest]
fn wrap_int_fill() {
    let bins_per_day = 24;
    let hours_per_bin = 1;
    let start_at_zero = 0;
    let mut hist =
        ndhistogram!(
            UniformCyclic::with_step_size(bins_per_day, start_at_zero, hours_per_bin).unwrap()
        );
    hist.fill(&40); // The 40th hour of the week ...
    assert_eq!(hist.value(&16), Some(&1.0)); // ... is at 4 pm.
}

#[rstest]
fn wrap_float_value() {
    let mut hist = ndhistogram!(UniformCyclic::new(4, 0.0, 360.0).unwrap(); u8);
    hist.fill(&45.0);
    assert_eq!(hist.value(&45.0), Some(&1));
    assert_eq!(hist.value(&(45.0 + 360.0)), Some(&1));
    assert_eq!(hist.value(&(45.0 - 360.0)), Some(&1));
}

#[rstest]
fn into_iter() {
    let axis = UniformCyclic::with_step_size(3, 15, 10).unwrap();
    let mut bins = vec![];
    for x in &axis {
        bins.push(x)
    }
    assert_eq!(bins[0], (0, BinInterval::new(15, 25)));
    assert_eq!(bins[1], (1, BinInterval::new(25, 35)));
    assert_eq!(bins[2], (2, BinInterval::new(35, 45)));
}
