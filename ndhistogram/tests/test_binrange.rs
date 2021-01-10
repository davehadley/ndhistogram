use std::{
    convert::TryInto,
    ops::{Range, RangeFrom, RangeTo},
};

use ndhistogram::axis::bininterval::BinInterval;

#[test]
fn test_binrange_equality() {
    let underflow0a = BinInterval::underflow(0.0);
    let underflow0b = BinInterval::underflow(0.0);
    let underflow1a = BinInterval::underflow(1.0);

    let overflow0a = BinInterval::overflow(0.0);
    let overflow0b = BinInterval::overflow(0.0);
    let overflow1a = BinInterval::overflow(1.0);

    let bin0a = BinInterval::new(0.0, 1.0);
    let bin0b = BinInterval::new(0.0, 1.0);
    let bin1a = BinInterval::new(1.0, 2.0);

    assert_eq!(underflow0a, underflow0b);
    assert_eq!(overflow0a, overflow0b);
    assert_eq!(bin0a, bin0b);

    assert_ne!(underflow0a, underflow1a);
    assert_ne!(overflow0a, overflow1a);
    assert_ne!(bin0a, bin1a);

    assert_ne!(underflow0a, overflow0a);
    assert_ne!(underflow0a, bin0a);
    assert_ne!(bin0a, overflow0a);
}

#[test]
fn test_binrange_underflow_debug() {
    let underflow = BinInterval::underflow(0.0);
    println!("{:?}", underflow);
}

#[test]
fn test_binrange_overflow_debug() {
    let overflow = BinInterval::overflow(0.0);
    println!("{:?}", overflow);
}

#[test]
fn test_binrange_bin_debug() {
    let bin = BinInterval::new(0.0, 1.0);
    println!("{:?}", bin);
}

#[test]
fn test_binrange_underflow_display() {
    let underflow = BinInterval::underflow(0.0);
    println!("{}", underflow);
}

#[test]
fn test_binrange_overflow_display() {
    let overflow = BinInterval::overflow(0.0);
    println!("{}", overflow);
}

#[test]
fn test_binrange_bin_display() {
    let bin = BinInterval::new(0.0, 1.0);
    println!("{}", bin);
}

#[test]
#[allow(clippy::float_cmp)]
fn test_binrange_bin_conversion() {
    let start = 0.0;
    let end = 1.0;
    let bin = BinInterval::new(start, end);
    let range: Range<_> = bin.clone().try_into().unwrap();
    let bin2: BinInterval<_> = range.clone().into();
    assert_eq!(range.start, start);
    assert_eq!(range.end, end);
    assert_eq!(bin, bin2);
}

#[test]
#[allow(clippy::float_cmp)]
fn test_binrange_underflow_conversion() {
    let end = 1.0;
    let bin = BinInterval::underflow(end);
    let range: RangeTo<_> = bin.clone().try_into().unwrap();
    let bin2: BinInterval<_> = range.clone().into();
    assert_eq!(range.end, end);
    assert_eq!(bin, bin2);
}

#[test]
#[allow(clippy::float_cmp)]
fn test_binrange_overflow_conversion() {
    let start = 1.0;
    let bin = BinInterval::overflow(start);
    let range: RangeFrom<_> = bin.clone().try_into().unwrap();
    let bin2: BinInterval<_> = range.clone().into();
    assert_eq!(range.start, start);
    assert_eq!(bin, bin2);
}

#[test]
fn test_binrange_is_clone() {
    let bin = BinInterval::new(0.0, 1.0);
    assert_eq!(bin, bin.clone());
}

#[test]
fn test_binrange_start_method() {
    let underflow = BinInterval::underflow(0.0);
    let overflow = BinInterval::overflow(1.0);
    let bin = BinInterval::new(0.0, 1.0);
    assert_eq!(bin.start(), Some(0.0));
    assert_eq!(underflow.start(), None);
    assert_eq!(overflow.start(), Some(1.0));
}

#[test]
fn test_binrange_end_method() {
    let underflow = BinInterval::underflow(0.0);
    let overflow = BinInterval::overflow(1.0);
    let bin = BinInterval::new(0.0, 1.0);
    assert_eq!(bin.end(), Some(1.0));
    assert_eq!(underflow.end(), Some(0.0));
    assert_eq!(overflow.end(), None);
}
