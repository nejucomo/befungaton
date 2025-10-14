use crate::Glyph;
use crate::errors::InvalidChar;
use crate::geometry::Direction;

use self::Widget::*;

/// An object which modifies a [Cursor] when stepped on
#[derive(Copy, Clone, Debug, Default)]
pub enum Widget {
    /// `' '`: do nothing
    #[default]
    Noop,
    /// `^ > v <`: turn in the given direction
    Turn(Direction),
}

impl TryFrom<char> for Widget {
    type Error = InvalidChar;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            ' ' => Ok(Noop),
            c => Direction::try_from(c).map(Turn),
        }
    }
}

impl Glyph for Widget {
    fn glyph(&self) -> char {
        match self {
            Noop => ' ',
            Turn(d) => d.glyph(),
        }
    }
}
