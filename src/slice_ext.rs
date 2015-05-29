use std::ops::*;

pub trait SliceExt<R: ?Sized> {
    fn slice<'a>(&'a self, start: usize, end: usize) -> &'a R;
}

impl<R: ?Sized, T> SliceExt<R> for T
where T: Index<RangeTo<usize>, Output=R>,
      R: Index<RangeFrom<usize>, Output=R> {

    fn slice(&self, start: usize, end: usize) -> &R {
        self.index(RangeTo { end: end }).index(RangeFrom { start: start })
    }
}
