use derive_new::new;

use std::fmt::{Debug, Display};
use std::ops::Add;

use crate::geometry::Direction;

/// A [Position] within a [Space](crate::Space)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, new)]
pub struct Position {
    /// The column
    pub col: i32,
    /// The row
    pub row: i32,
}

impl Position {
    /// Construct a new [Position]
    ///
    /// # Note
    ///
    /// The `From<(I, I)>` impl for [Position] delegates to this method.
    ///
    /// # Panics
    ///
    /// Panics if there are integer conversion overflows
    pub fn new_conv<I>(col: I, row: I) -> Self
    where
        i32: TryFrom<I>,
        <i32 as TryFrom<I>>::Error: Debug,
    {
        Self::try_new(col, row).unwrap()
    }

    /// Attempt to construct a [Position], returning an error on overflows
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

impl<I> From<(I, I)> for Position
where
    i32: TryFrom<I>,
    <i32 as TryFrom<I>>::Error: Debug,
{
    fn from((col, row): (I, I)) -> Self {
        Self::new_conv(col, row)
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
