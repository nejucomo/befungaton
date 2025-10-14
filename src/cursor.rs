use crate::geometry::Direction;
use crate::{Glyph, Widget};

use derive_new::new;

/// A [Cursor] defines a code execution context
#[derive(Debug, new)]
pub struct Cursor {
    /// The [Direction] the [Cursor] points in
    pub direction: Direction,
    /// A stack of values for the execution context
    #[new(default)]
    pub stack: Vec<Widget>,
}

impl Glyph for Cursor {
    fn glyph(&self) -> char {
        use Direction::*;

        match self.direction {
            North => '▲',
            South => '▼',
            East => '▶',
            West => '◀',
        }
    }
}
