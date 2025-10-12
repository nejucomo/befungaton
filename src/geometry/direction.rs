use crate::Glyph;
use crate::errors::InvalidChar;

use self::Direction::*;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl TryFrom<char> for Direction {
    type Error = InvalidChar;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(North),
            'v' => Ok(South),
            '>' => Ok(East),
            '<' => Ok(West),
            _ => Err(InvalidChar(c)),
        }
    }
}

impl Glyph for Direction {
    fn glyph(&self) -> char {
        match *self {
            North => '^',
            South => 'v',
            East => '>',
            West => '<',
        }
    }
}
