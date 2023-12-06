pub mod day2;

/// Even more iterator stuff that I needed and isn't in itertools (or I haven't found it yet)
pub trait MoreItertools {
    /// Parses the remaining elements in this iterator as an integer
    fn parse_int(&mut self) -> i64
    where
        Self: Iterator<Item = u8>;
}

impl<T: Iterator> MoreItertools for T {
    fn parse_int(&mut self) -> i64
    where
        T: Iterator<Item = u8>,
    {
        self.fold(0i64, |p, c| p * 10 + (c - b'0') as i64)
    }
}
