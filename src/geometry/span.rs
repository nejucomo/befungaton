use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Neg, Range, Sub};

use derive_more::From;
use derive_new::new;

/// A type is a [Spanner] if it can be useas a Span boundary
pub trait Spanner:
    Copy
    + Display
    + Debug
    + Ord
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Div<i32, Output = Self>
{
    /// The next larger value
    fn one() -> Self;
}

/// A range of values which can be extended to cover samples
///
/// # Relation to `std::ops::Range`
///
/// A [Span] is very similar to a [Range] with these notable differences:
///
/// - it must be [Copy]
/// - it has methods specific to this app
#[derive(Copy, Clone, Debug, Default, From, new)]
pub struct Span<T>
where
    T: Spanner,
{
    /// The low end of the span
    lo: T,
    /// The hi end of the span (exclusive, similar to [Range])
    hi: T,
}

impl<T> Span<T>
where
    T: Spanner,
{
    /// The inclusive minimum value of `self`
    pub fn lo(&self) -> T {
        self.lo
    }

    /// The exclusive maximum value of `self`
    pub fn hi(&self) -> T {
        self.hi
    }

    /// The magnitude from low to high: `self.hi() - self.lo()`
    pub fn magnitude(self) -> T {
        self.hi - self.lo
    }

    /// The center of the span
    pub fn center(&self) -> T {
        self.lo + self.magnitude() / 2
    }

    /// Modify this span to ensure it includes `sample`
    pub fn extend_to_cover(&mut self, sample: T)
    where
        T: Spanner,
    {
        use std::cmp::{max, min};

        self.lo = min(self.lo, sample);
        self.hi = max(self.hi, sample + T::one());
    }
}

impl<T> std::fmt::Display for Span<T>
where
    T: Spanner,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.lo, self.hi)
    }
}

impl<T> From<Range<T>> for Span<T>
where
    T: Spanner,
{
    fn from(rng: Range<T>) -> Self {
        Span::new(rng.start, rng.end)
    }
}

impl<T> IntoIterator for &Span<T>
where
    T: Spanner,
    Range<T>: Iterator<Item = T>,
{
    type Item = T;
    type IntoIter = impl Iterator<Item = T>;

    fn into_iter(self) -> Self::IntoIter {
        self.lo..self.hi
    }
}
