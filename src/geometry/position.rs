use derive_new::new;
use std::fmt::Display;
use std::ops::Add;

use crate::geometry::Direction;

#[derive(Copy, Clone, Debug, new, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub col: i32,
    pub row: i32,
}

impl Position {
    pub fn try_new<I>(col: I, row: I) -> Result<Self, <i32 as TryFrom<I>>::Error>
    where
        i32: TryFrom<I>,
    {
        Ok(Position {
            col: i32::try_from(col)?,
            row: i32::try_from(row)?,
        })
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, d: Direction) -> Self::Output {
        use Direction::*;

        let (dcol, drow) = match d {
            North => (0, -1),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
        };

        Position {
            col: self.col + dcol,
            row: self.row + drow,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "row {}, col {}", self.row, self.col)
    }
}
