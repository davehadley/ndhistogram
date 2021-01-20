use ndhistogram::{axis::Uniform, ndhistogram, Histogram};

#[test]
fn test_histogram_display_blank() {
    let hist = ndhistogram!(Uniform::new(4, 0.0, 2.0));
    let actual = format!("{}", hist);
    println!("{}", actual);
    let expected = "VecHistogram1D(6 bins, sum=0)
    (-inf, 0.00) | 
    [0.00, 0.50) | 
    [0.50, 1.00) | 
    [1.00, 1.50) | 
    [1.50, 2.00) | 
     [2.00, inf) | ";
    assert_eq!(actual, expected);
}

#[test]
fn test_histogram_display_filled_positive() {
    let mut hist = ndhistogram!(Uniform::new(4, 0.0, 2.0));
    vec![
        (-1.0, 2.0),
        (0.1, 0.9),
        (0.6, 4.0),
        (1.1, 6.0),
        (1.6, 100.0),
        (2.1, 50.0),
    ]
    .iter()
    .for_each(|(x, w)| hist.fill_with(x, w));
    let actual = format!("{}", hist);
    println!("{}", actual);
    let expected = "VecHistogram1D(6 bins, sum=162.9)
    (-inf, 0.00) | #
    [0.00, 0.50) | 
    [0.50, 1.00) | ##
    [1.00, 1.50) | ###
    [1.50, 2.00) | ##################################################
     [2.00, inf) | #########################";
    assert_eq!(actual, expected);
}
