use derive_new::new;

use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Neg, Sub};

/// A [Position] within a [Space](crate::Space)
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, new)]
pub struct Position {
    /// The column
    pub col: i32,
    /// The row
    pub row: i32,
}

impl Position {
    /// The position at (1, 1)
    pub fn one_one() -> Self {
        Position::new(1, 1)
    }

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

    /// Return either `self` or `other`, whichever is more towards the bottom and/or right
    pub fn more_bottom_right(self, other: Position) -> Position {
        use std::cmp::max;

        Position::new(max(self.col, other.col), max(self.row, other.row))
    }

    /// Return either `self` or `other`, whichever is more towards the top and/or left
    pub fn more_top_left(self, other: Position) -> Position {
        use std::cmp::min;

        Position::new(min(self.col, other.col), min(self.row, other.row))
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

impl<I> TryFrom<Position> for (I, I)
where
    I: TryFrom<i32>,
{
    type Error = <I as TryFrom<i32>>::Error;

    fn try_from(pos: Position) -> Result<Self, Self::Error> {
        let col = I::try_from(pos.col)?;
        let row = I::try_from(pos.row)?;
        Ok((col, row))
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            col: self.col + rhs.col,
            row: self.row + rhs.row,
        }
    }
}

impl Neg for Position {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Position {
            col: -self.col,
            row: -self.row,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Div<i32> for Position {
    type Output = Position;

    fn div(self, rhs: i32) -> Self::Output {
        Position {
            col: self.col / rhs,
            row: self.row / rhs,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "row {}, col {}", self.row, self.col)
    }
}
