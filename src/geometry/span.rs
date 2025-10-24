use std::ops::Range;

use derive_more::From;
use derive_new::new;

use crate::geometry::Position;

/// A range of values which can be extended to cover samples
///
/// # Relation to `std::ops::Range`
///
/// A [Span] is very similar to a [Range] with these notable differences:
///
/// - it must be [Copy]
/// - it has methods specific to this app
#[derive(Copy, Clone, Debug, Default, From, new)]
pub struct Span<T> {
    /// The low end of the span
    pub lo: T,
    /// The hi end of the span (exclusive, simimplar to [Range])
    pub hi: T,
}

/// A rectangle defined by top-left and lower-right [Position]s
pub type Rect = Span<Position>;

impl<T> Span<T> {
    /// Modify this span to ensure it includes `sample`
    pub fn extend_to_cover(&mut self, sample: T)
    where
        T: Copy + Ord,
    {
        use std::cmp::{max, min};

        self.lo = min(self.lo, sample);
        self.hi = max(self.hi, sample);
    }
}

impl Rect {
    /// An iterator over the column indices
    pub fn cols(&self) -> impl Iterator<Item = i32> {
        self.lo.col..self.hi.col + 1
    }

    /// An iterator over the row indices
    pub fn rows(&self) -> impl Iterator<Item = i32> {
        self.lo.row..self.hi.row + 1
    }
}

impl<T> From<Range<T>> for Span<T> {
    fn from(rng: Range<T>) -> Self {
        Span::new(rng.start, rng.end)
    }
}

impl<T> IntoIterator for &Span<T>
where
    T: Copy,
    Range<T>: Iterator<Item = T>,
{
    type Item = T;
    type IntoIter = impl Iterator<Item = T>;

    fn into_iter(self) -> Self::IntoIter {
        self.lo..self.hi
    }
}
