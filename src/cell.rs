use crate::Direction::{self, East, North, South, West};

use self::Cell::*;

/// Every cell can be represented as a unicode scalar value
#[derive(Copy, Clone, Debug, Default)]
pub enum Cell {
    #[default]
    Noop,
    Turn(Direction),
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            ' ' => Ok(Noop),
            '^' => Ok(Turn(North)),
            '>' => Ok(Turn(East)),
            'v' => Ok(Turn(South)),
            '<' => Ok(Turn(West)),
            _ => Err(()),
        }
    }
}
