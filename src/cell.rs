use crate::Glyph;
use crate::errors::InvalidChar;
use crate::geometry::Direction;

use self::Cell::*;

/// Every cell can be represented as a unicode scalar value
#[derive(Copy, Clone, Debug, Default)]
pub enum Cell {
    #[default]
    Noop,
    Turn(Direction),
}

impl TryFrom<char> for Cell {
    type Error = InvalidChar;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            ' ' => Ok(Noop),
            c => Direction::try_from(c).map(Turn),
        }
    }
}

impl Glyph for Cell {
    fn glyph(&self) -> char {
        match self {
            Noop => ' ',
            Turn(d) => d.glyph(),
        }
    }
}
