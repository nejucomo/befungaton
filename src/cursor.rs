use crate::geometry::Direction;
use crate::{Glyph, Widget};

use derive_new::new;

#[derive(Debug, new)]
pub struct Cursor {
    pub direction: Direction,
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
