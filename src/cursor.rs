use crate::geometry::{Direction, Position};
use crate::{Cell, Glyph};

use derive_new::new;

#[derive(Debug, new)]
pub struct Cursor {
    pub position: Position,
    pub direction: Direction,
    #[new(default)]
    pub stack: Vec<Cell>,
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
