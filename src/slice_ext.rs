use std::ops::*;

pub trait SliceExt {
    fn slice<'a>(&'a self, start: usize, end: usize) -> &'a str;
}

impl<T> SliceExt for T
where T: Index<RangeTo<usize>, Output=str> {

    fn slice(&self, start: usize, end: usize) -> &str {
        self.index(RangeTo { end: end }).index(RangeFrom { start: start })
    }
}
