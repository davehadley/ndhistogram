use ndhistogram::axis::SingleValueBinInterval;

#[test]
fn test_categorybinrange_equality() {
    let overflow0a = SingleValueBinInterval::overflow();
    let overflow0b = SingleValueBinInterval::overflow();

    let bin0a = SingleValueBinInterval::new("Zero");
    let bin0b = SingleValueBinInterval::new("Zero");
    let bin1a = SingleValueBinInterval::new("One");

    assert_eq!(overflow0a, overflow0b);

    assert_eq!(bin0a, bin0b);
    assert_ne!(bin0a, bin1a);
    assert_ne!(bin0a, overflow0a);
}

#[test]
fn test_categorybinrange_overflow_debug() {
    let overflow = SingleValueBinInterval::<String>::overflow();
    println!("{:?}", overflow);
}

#[test]
fn test_categorybinrange_bin_debug() {
    let bin = SingleValueBinInterval::new("One");
    println!("{:?}", bin);
}

#[test]
fn test_categorybinrange_overflow_display() {
    let overflow = SingleValueBinInterval::<String>::overflow();
    println!("{}", overflow);
}

#[test]
fn test_categorybinrange_bin_display() {
    let bin = SingleValueBinInterval::new("One");
    println!("{}", bin);
}

#[test]
fn test_categorybinrange_is_clone() {
    let bin = SingleValueBinInterval::new("One");
    assert_eq!(bin, bin.clone());
}

#[test]
fn test_categorybinrange_value_method() {
    let overflow = SingleValueBinInterval::<String>::overflow();
    let bin = SingleValueBinInterval::new("One");
    assert_eq!(bin.value(), Some(&"One"));
    assert_eq!(overflow.value(), None);
}
