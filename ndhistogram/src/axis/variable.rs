use super::{Axis, BinInterval};

#[derive(Debug, Clone)]
pub struct Variable<T = f64> {
    bin_edges: Vec<T>,
}

impl<T> Variable<T>
where
    T: PartialOrd,
{
    pub fn new<I: IntoIterator<Item = T>>(bin_edges: I) -> Self {
        let mut bin_edges: Vec<T> = bin_edges.into_iter().collect();
        if bin_edges.len() < 2 {
            panic!("Invalid axis number of bin edges ({})", bin_edges.len());
        }
        bin_edges.sort_by(|a, b| a.partial_cmp(b).expect("failed to sort bin_edges."));
        Self { bin_edges }
    }

    /// Low edge of axis (excluding underflow bin).
    pub fn low(&self) -> &T {
        &self.bin_edges.first().unwrap()
    }

    /// High edge of axis (excluding overflow bin).
    pub fn high(&self) -> &T {
        &self.bin_edges.last().unwrap()
    }
}

impl<T> Axis for Variable<T>
where
    T: PartialOrd,
{
    type Coordinate = T;
    type BinInterval = BinInterval<T>;

    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
        match self.bin_edges.binary_search_by(|probe| {
            probe
                .partial_cmp(&coordinate)
                .expect("incomparable values. NAN bin edges?")
        }) {
            Ok(index) => Some(index + 1),
            Err(index) => Some(index),
        }
    }

    fn numbins(&self) -> usize {
        self.bin_edges.len() + 1
    }

    fn bin(&self, index: usize) -> Option<Self::BinInterval> {
        todo!()
    }
}
