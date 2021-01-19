use ndhistogram::{
    axis::{Axis, Category},
    ndhistogram, AxesTuple, Histogram, Item, VecHistogram,
};

#[test]
fn test_histogram_category_1d_unweighted_fill_once() {
    let mut hist = ndhistogram!(Category::new(vec!["A", "B"]); i32);
    hist.fill(&"A");
    let actual = *hist.value(&"A").unwrap();
    let expected = 1;
    assert_eq!(expected, actual);
}

#[test]
fn test_histogram_category_1d_unfilled_is_empty() {
    let hist = ndhistogram!(Category::new(vec!["A", "B"]); i32);
    let actual: Vec<_> = hist.values().copied().collect();
    let expected = vec![0, 0, 0];
    assert_eq!(expected, actual);
}

#[test]
fn test_histogram_category_1d_weighted_fill_once() {
    let mut hist = ndhistogram!(Category::new(vec!["A", "B"]); i32);
    hist.fill_with(&"A", 2);
    let actual = *hist.value(&"A").unwrap();
    let expected = 2;
    assert_eq!(expected, actual);
}

#[test]
#[allow(clippy::float_cmp)]
fn test_histogram_category_1d_multifill() {
    let mut hist = ndhistogram!(Category::new(vec!["A", "B"]); i32);
    hist.fill(&"A");
    hist.fill_with(&"B", 2);
    hist.fill_with(&"C", 3);
    let actual: Vec<_> = hist.values().copied().collect();
    assert_eq!(actual, vec![1, 2, 3]);
}

#[test]
fn test_histogram_category_1d_get_axes() {
    let ax = Category::new(vec!["A", "B"]);
    let hist = ndhistogram!(ax.clone());
    let axtuple = hist.axes();
    assert_eq!(ax, axtuple.as_tuple().0);
}

#[test]
fn test_histogram_category_1d_value_at_index() {
    let mut hist = ndhistogram!(Category::new(vec!["A", "B"]); i32);
    hist.fill(&"A");
    assert_eq!(hist.value_at_index(0), Some(&1));
    assert_eq!(hist.value_at_index(1), Some(&0));
    assert_eq!(hist.value_at_index(2), Some(&0));
    assert_eq!(hist.value_at_index(3), None);
}

#[test]
fn test_histogram_category_1d_value_at_coordinate() {
    let mut hist = ndhistogram!(Category::new(vec!["A", "B"]); i32);
    hist.fill(&"A");
    assert_eq!(hist.value(&"A"), Some(&1));
    assert_eq!(hist.value(&"B"), Some(&0));
    assert_eq!(hist.value(&"C"), Some(&0));
}

fn make_simple_cat_histogram() -> VecHistogram<AxesTuple<(Category<&'static str>,)>, i32> {
    let mut hist = ndhistogram!(Category::new(vec!["A", "B"]); i32);
    hist.fill(&"A");
    hist.fill_with(&"B", 2);
    hist.fill_with(&"C", 3);
    hist
}

#[test]
fn test_histogram_value_iterator() {
    let hist = make_simple_cat_histogram();
    let actual: Vec<_> = hist.values().collect();
    let expected = vec![&1, &2, &3];
    assert_eq!(actual, expected);
}

#[test]
fn test_histogram_item_iterator() {
    let hist = make_simple_cat_histogram();
    let actual: Vec<_> = hist.iter().collect();
    let expected: Vec<_> = (0..3)
        .map(|it| {
            Item::new(
                it,
                hist.axes().bin(it).unwrap(),
                hist.value_at_index(it).unwrap(),
            )
        })
        .collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_histogram_category_1d_value_at_index_mut() {
    let mut hist = ndhistogram!(Category::new(vec!["A", "B"]); i32);
    *(hist.value_at_index_mut(1).unwrap()) = 2;
    assert_eq!(hist.value_at_index(0), Some(&0));
    assert_eq!(hist.value_at_index(1), Some(&2));
    assert_eq!(hist.value_at_index(2), Some(&0));
    assert_eq!(hist.value_at_index(3), None);
}

#[test]
fn test_histogram_category_1d_value_mut() {
    let mut hist = ndhistogram!(Category::new(vec!["A", "B"]); i32);
    *(hist.value_mut(&"B").unwrap()) = 2;
    assert_eq!(hist.value_at_index(0), Some(&0));
    assert_eq!(hist.value_at_index(1), Some(&2));
    assert_eq!(hist.value_at_index(2), Some(&0));
    assert_eq!(hist.value_at_index(3), None);
}

#[test]
fn test_histogram_category_1d_iter_values_mut() {
    let mut hist = ndhistogram!(Category::new(vec!["A", "B"]); i32);
    hist.values_mut()
        .enumerate()
        .for_each(|(index, value)| *value = index as i32);
    assert_eq!(hist.value_at_index(0), Some(&0));
    assert_eq!(hist.value_at_index(1), Some(&1));
    assert_eq!(hist.value_at_index(2), Some(&2));
    assert_eq!(hist.value_at_index(3), None);
}

#[test]
fn test_histogram_category_1d_iter_mut() {
    let mut hist = ndhistogram!(Category::new(vec!["A", "B"]); i32);
    (&mut hist)
        .into_iter()
        .for_each(|item| *item.value = item.index as i32);
    assert_eq!(hist.value_at_index(0), Some(&0));
    assert_eq!(hist.value_at_index(1), Some(&1));
    assert_eq!(hist.value_at_index(2), Some(&2));
    assert_eq!(hist.value_at_index(3), None);
}
