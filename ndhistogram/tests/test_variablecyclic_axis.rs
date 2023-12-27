use ndhistogram::axis::{Axis, BinInterval, VariableCyclic};

use rstest::rstest;

fn bint<T>(lo: T, hi: T) -> Option<BinInterval<T>> {
    Some(BinInterval::new(lo, hi))
}

#[rstest(/**/ bin_no, expected_interval, edges,
         case(0, bint(0.0, 0.5), &[0.0, 0.5, 2.0]),
         case(0, bint(0.0, 0.5), &[0.5, 2.0, 0.0]),
         case(1, bint(0.5, 2.0), &[0.0, 0.5, 2.0]),
         case(1, bint(0.5, 2.0), &[0.5, 2.0, 0.0]),
)]
fn bin_float(bin_no: usize, expected_interval: Option<BinInterval<f32>>, edges: &[f32]) {
    let axis = VariableCyclic::new(edges.iter().cloned()).unwrap();
    assert_eq!(axis.bin(bin_no), expected_interval);
}

#[rstest(/**/ bin_no, expected_interval, edges,
         case(0, bint(0,  5), &[ 0,  5, 20]),
         case(0, bint(0,  5), &[ 5, 20,  0]),
         case(1, bint(5, 20), &[ 0,  5, 20]),
         case(1, bint(5, 20), &[ 5, 20,  0]),
)]
fn bin_int(bin_no: usize, expected_interval: Option<BinInterval<i32>>, edges: &[i32]) {
    let axis = VariableCyclic::new(edges.iter().cloned()).unwrap();
    assert_eq!(axis.bin(bin_no), expected_interval);
}

#[rstest(/**/coordinate, expected_index,          edges,
         case(0.5      ,     Some(0)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
         case(1.5      ,     Some(1)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
         case(3.0      ,     Some(2)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
         case(6.0      ,     Some(3)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
         case(0.5 + 8.0,     Some(0)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
         case(1.5 + 8.0,     Some(1)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
         case(3.0 + 8.0,     Some(2)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
         case(6.0 + 8.0,     Some(3)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
         case(0.5 - 8.0,     Some(0)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
         case(1.5 - 8.0,     Some(1)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
         case(3.0 - 8.0,     Some(2)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
         case(6.0 - 8.0,     Some(3)   , &[0.0, 1.0, 2.0, 4.0, 8.0]),
)]
fn float_index(coordinate: f32, expected_index: Option<usize>, edges: &[f32]) {
    let axis = VariableCyclic::new(edges.iter().cloned()).unwrap();
    assert_eq!(axis.index(&coordinate), expected_index);
}

#[rstest(/**/coordinate, expected_index,          edges,
         case( 5       ,     Some(0)   , &[ 0, 10, 20, 40, 80]),
         case(15       ,     Some(1)   , &[ 0, 10, 20, 40, 80]),
         case(30       ,     Some(2)   , &[ 0, 10, 20, 40, 80]),
         case(60       ,     Some(3)   , &[ 0, 10, 20, 40, 80]),
         case( 5 + 80  ,     Some(0)   , &[ 0, 10, 20, 40, 80]),
         case(15 + 80  ,     Some(1)   , &[ 0, 10, 20, 40, 80]),
         case(30 + 80  ,     Some(2)   , &[ 0, 10, 20, 40, 80]),
         case(60 + 80  ,     Some(3)   , &[ 0, 10, 20, 40, 80]),
         case( 5 - 80  ,     Some(0)   , &[ 0, 10, 20, 40, 80]),
         case(15 - 80  ,     Some(1)   , &[ 0, 10, 20, 40, 80]),
         case(30 - 80  ,     Some(2)   , &[ 0, 10, 20, 40, 80]),
         case(60 - 80  ,     Some(3)   , &[ 0, 10, 20, 40, 80]),
)]
fn int_index(coordinate: i32, expected_index: Option<usize>, edges: &[i32]) {
    let axis = VariableCyclic::new(edges.iter().cloned()).unwrap();
    assert_eq!(axis.index(&coordinate), expected_index);
}

#[rstest(/**/ edges,
         case(&[1.0, 2.0]),
         case(&[8.5, 2.3, 9.4, -23.4]),
)]
fn indices(edges: &[f32]) {
    let n = edges.len() - 1;
    let axis = VariableCyclic::new(edges.iter().cloned()).unwrap();
    let indices = axis.indices().collect::<Vec<_>>();
    assert_eq!(indices, (0..n).collect::<Vec<_>>());
}

use ndhistogram::{ndhistogram, Histogram};

#[rstest]
fn wrap_float_fill() {
    let mut hist = ndhistogram!(VariableCyclic::new(vec![8.0, 0.0, 4.0, 2.0]).unwrap(); u8);
    let v = &5.0;
    let r = 8.0;
    hist.fill(v);
    hist.fill(&(v + r));
    hist.fill(&(v - r));
    assert_eq!(hist.value(v), Some(&3));
    assert_eq!(hist.value_at_index(2), Some(&3));
}

#[rstest]
fn wrap_int_fill() {
    let mut hist = ndhistogram!(VariableCyclic::new(vec![8, 0, 4, 2]).unwrap(); u8);
    let v = &5;
    let r = 8;
    hist.fill(v);
    hist.fill(&(v + r));
    hist.fill(&(v - r));
    assert_eq!(hist.value(v), Some(&3));
    assert_eq!(hist.value_at_index(2), Some(&3));
}

#[rstest]
fn wrap_float_value() {
    let mut hist = ndhistogram!(VariableCyclic::new(vec![4.0, 8.0, 2.0, 1.0]).unwrap(); u8);
    let v = &2.3;
    let r = 7.0;
    hist.fill(v);
    assert_eq!(hist.value(v), Some(&1));
    assert_eq!(hist.value(&(v - r)), Some(&1));
    assert_eq!(hist.value(&(v + r)), Some(&1));
}

#[rstest]
fn into_iter() {
    let axis = VariableCyclic::new([0, 1, 10, 100]).unwrap();
    let mut bins = vec![];
    for x in &axis {
        bins.push(x)
    }
    assert_eq!(bins[0], (0, BinInterval::new(0, 1)));
    assert_eq!(bins[1], (1, BinInterval::new(1, 10)));
    assert_eq!(bins[2], (2, BinInterval::new(10, 100)));
}
