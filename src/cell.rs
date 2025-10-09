use crate::{CursorState, Direction, Glyph, GlyphStyle};

use self::Cell::*;

/// A Cell is isomorphic to a unicode scalar value
#[derive(Debug)]
pub enum Cell {
    Inert(char),
    Cursor(CursorState),
}

impl Default for Cell {
    fn default() -> Self {
        Inert(' ')
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        if let Ok(d) = Direction::try_from(c) {
            Cursor(CursorState::new(d))
        } else {
            Inert(c)
        }
    }
}

impl Glyph for Cell {
    fn glyph(&self, style: GlyphStyle) -> char {
        match self {
            Inert(c) => *c,
            Cursor(st) => st.direction.glyph(style),
        }
    }
}
