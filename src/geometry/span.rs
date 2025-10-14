use std::ops::Range;

use derive_more::{Deref, From};

/// A range of values which can be extended to cover samples
#[derive(Debug, Default, Deref, From)]
pub struct Span(Range<i32>);

/// Any type which can be extended to cover samples (e.g. [Span] or [Rect](crate::geometry::Rect)
pub trait Spanning<T> {
    /// Extend the [Spanning] value to cover `sample`
    fn extend_to_cover(&mut self, sample: T);
}

impl Spanning<i32> for Span {
    fn extend_to_cover(&mut self, sample: i32) {
        use std::cmp::{max, min};

        let &Range { start, end } = &self.0;

        self.0 = min(start, sample)..max(end, sample + 1);
    }
}

impl IntoIterator for &Span {
    type Item = i32;
    type IntoIter = <Range<i32> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.clone()
    }
}
