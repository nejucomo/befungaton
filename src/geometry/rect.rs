use crate::geometry::Position;

/// A rectangle defined by top-left and lower-right [Position]s
#[derive(Debug, Default)]
pub struct Rect {
    /// Top-Left Inclusive
    tli: Position,
    /// Bottom-Right Exclusive
    brx: Position,
}

impl Rect {
    // /// The magnitude from low to high: `self.hi() - self.lo()`
    // pub fn magnitude(self) -> Position {
    //     self.brx - self.tli
    // }

    // /// The center of the span
    // pub fn center(&self) -> Position {
    //     self.tli + self.magnitude() / 2
    // }

    /// An iterator over the column indices
    pub fn cols(&self) -> impl Iterator<Item = i32> {
        self.tli.col..self.brx.col
    }

    /// An iterator over the row indices
    pub fn rows(&self) -> impl Iterator<Item = i32> {
        self.tli.row..self.brx.row
    }

    /// Extend if necessary to ensure `sample` is contained by `self`
    pub fn extend_to_cover(&mut self, sample: Position) {
        self.tli = self.tli.more_top_left(sample);
        self.brx = self.brx.more_bottom_right(sample + Position::one_one());
    }
}
