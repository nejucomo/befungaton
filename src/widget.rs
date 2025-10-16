mod digit;

use crate::Glyph;
use crate::errors::InvalidChar;
use crate::geometry::Direction;

pub use self::digit::Digit;

use self::Widget::*;

/// An object which modifies a [Cursor](crate::Cursor) when stepped on
#[derive(Copy, Clone, Debug, Default)]
pub enum Widget {
    /// `' '`: do nothing
    #[default]
    Noop,
    /// `G`: turn counter-clockwise
    Ccw,
    /// `=`: if stack is empty, turn south; else pop, if nonzero turn north.
    TurnIfZero,
    /// `^ > v <`: turn in the given direction
    Turn(Direction),
    /// `0 1 2 3 4 5 6 7 8 9`: push the given number onto the stack
    PushDigit(Digit),
}

impl TryFrom<char> for Widget {
    type Error = InvalidChar;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            ' ' => Ok(Noop),
            'G' => Ok(Ccw),
            '=' => Ok(TurnIfZero),
            c => Digit::try_from(c)
                .map(PushDigit)
                .or_else(|_| Direction::try_from(c).map(Turn)),
        }
    }
}

impl Glyph for Widget {
    fn glyph(&self) -> char {
        match self {
            Noop => ' ',
            Ccw => 'G',
            TurnIfZero => '=',
            Turn(d) => d.glyph(),
            PushDigit(d) => d.glyph(),
        }
    }
}
