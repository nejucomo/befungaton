use derive_new::new;
use std::fmt::Display;

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

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "row {}, col {}", self.row, self.col)
    }
}
