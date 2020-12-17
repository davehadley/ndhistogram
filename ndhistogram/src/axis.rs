pub trait Axis {
    type Coordinate;
    fn index(&self, coordinate: &Self::Coordinate) -> usize;
    fn numbins(&self) -> usize;
    fn size(&self) -> usize {
        self.numbins() + 2
    } // includes overflow and underflow
}

struct Uniform {
    num: usize,
    low: f64,
    high: f64,
}

impl Axis for Uniform {
    type Coordinate = f64;

    fn index(&self, coordinate: &Self::Coordinate) -> usize {
        let frac = (coordinate - self.low) / (self.high - self.low);
        if frac < 0.0 {
            return 0;
        } else if frac >= 1.0 {
            return self.num + 1 as usize;
        }
        let idx = (self.num as f64) * frac;
        return (idx as usize) + 1;
    }

    fn numbins(&self) -> usize {
        self.num + 2
    }
}
