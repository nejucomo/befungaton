use std::ops::Add;

use crate::Glyph;
use crate::errors::InvalidChar;
use crate::geometry::Position;

use self::Direction::*;

/// A cardinal [Direction] in space
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    /// [North] is the direction of decreasing rows
    North,
    /// [South] is the direction of increasing rows
    South,
    /// [East] is the direction of increasing columns
    East,
    /// [West] is the direction of decreasing columns
    West,
}

impl Direction {
    /// The direction from turning counterclockwise
    pub fn counterclockwise(self) -> Direction {
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
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

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, d: Direction) -> Self::Output {
        self + Position::from(match d {
            North => (0, -1),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
        })
    }
}
