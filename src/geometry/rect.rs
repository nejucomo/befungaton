use crate::geometry::{Position, Span};

/// A rectangle defined by top-left and lower-right [Position]s
pub type Rect = Span<Position>;

impl Rect {
    /// An iterator over the column indices
    pub fn cols(&self) -> impl Iterator<Item = i32> {
        self.lo().col..self.hi().col
    }

    /// An iterator over the row indices
    pub fn rows(&self) -> impl Iterator<Item = i32> {
        self.lo().row..self.hi().row
    }
}
