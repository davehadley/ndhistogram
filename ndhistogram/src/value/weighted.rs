#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Weighted<T, W> {
    pub value: T,
    pub weight: W,
}

impl<T, W> Weighted<T, W> {
    pub fn new(value: T, weight: W) -> Self {
        Self { value, weight }
    }
}
