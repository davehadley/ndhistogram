mod bininterval;
pub use bininterval::bininterval::BinInterval;
pub use bininterval::singlevaluebininterval::SingleValueBinInterval;
mod uniform;
pub use uniform::Uniform;
mod uniformnoflow;
pub use uniformnoflow::UniformNoFlow;
mod category;
pub use category::Category;
mod categorynoflow;
pub use categorynoflow::CategoryNoFlow;

type Iter<'a, BinInterval> = Box<dyn Iterator<Item = (usize, BinInterval)> + 'a>;
type Indices = Box<dyn Iterator<Item = usize>>;
type Bins<'a, BinInterval> = Box<dyn Iterator<Item = BinInterval> + 'a>;

pub trait Axis {
    type Coordinate;
    type BinInterval;

    fn index(&self, coordinate: &Self::Coordinate) -> Option<usize>;
    fn numbins(&self) -> usize;

    fn bin(&self, index: usize) -> Option<Self::BinInterval>;

    fn indices(&self) -> Indices {
        Box::new(0..self.numbins())
    }

    fn iter(&self) -> Iter<'_, Self::BinInterval> {
        Box::new(self.indices().map(move |it| (it, self.bin(it).unwrap())))
    }

    fn bins(&self) -> Bins<'_, Self::BinInterval> {
        Box::new(self.indices().map(move |it| self.bin(it).unwrap()))
    }
}
