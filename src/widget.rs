use crate::errors::InvalidChar;
use crate::geometry::Direction;
use crate::{Glyph, Physical};

use self::Widget::*;

/// Every cell can be represented as a unicode scalar value
#[derive(Copy, Clone, Debug, Default)]
pub enum Widget {
    #[default]
    Noop,
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
