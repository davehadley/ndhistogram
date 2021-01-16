/// Intended for use with [FillWith] in cases where a weight is used in
/// addition to some other data. For an example usage see
/// [WeightedMean](crate::value::WeightedMean).
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Weighted<T, W> {
    /// The value being wrapped.
    pub value: T,
    /// The weight attached to this value.
    pub weight: W,
}

impl<T, W> Weighted<T, W> {
    /// Factory method to create Weighted.
    pub fn new(value: T, weight: W) -> Self {
        Self { value, weight }
    }
}
