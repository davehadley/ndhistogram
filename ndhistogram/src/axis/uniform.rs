use std::ops::Range;

use super::Axis;

pub struct Uniform {
    num: usize,
    low: f64,
    high: f64,
}

impl Uniform {
    pub fn new(num: usize, low: f64, high: f64) -> Uniform {
        if num == 0 {
            panic!("Invalid axis num bins ({})", num);
        }
        if low >= high {
            panic!("Invalid axis range bins (low:{}, high:{})", low, high);
        }
        Uniform { num, low, high }
    }
}

impl Axis for Uniform {
    type Coordinate = f64;
    type BinItem = Range<Self::Coordinate>;

    fn index(&self, coordinate: &Self::Coordinate) -> usize {
        let frac = (coordinate - self.low) / (self.high - self.low);
        if frac < 0.0 {
            return 0;
        } else if frac >= 1.0 {
            return self.num + 1 as usize;
        }
        let idx = (self.num as f64) * frac;
        (idx as usize) + 1
    }

    fn numbins(&self) -> usize {
        self.num
    }

    fn bin(&self, index: usize) -> std::option::Option<<Self as Axis>::BinItem> {
        if index == 0 || (index - 1) >= self.num {
            return None;
        }
        let interval = (self.high - self.low) / (self.num as f64);
        let start = ((index - 1) as f64) * (self.high - self.low) / (self.num as f64);
        let end = (index as f64) * (self.high - self.low) / (self.num as f64);
        Some(Range { start, end })
    }
}
