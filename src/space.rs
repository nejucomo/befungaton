#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::fmt::Display;

use crate::errors::SpaceFromStringError;
use crate::{Cell, Glyph as _, Position, Span};

#[derive(Debug, Default)]
pub struct Space {
    /// Invariant: [Cell] is never stored as [Cell::Noop] because that's represented by absence
    cells: BTreeMap<Position, Cell>,
    colspan: Span,
    rowspan: Span,
}

impl Space {
    fn set(&mut self, pos: Position, cell: Cell) {
        if matches!(cell, Cell::Noop) {
            self.colspan.extend_to_cover(pos.col);
            self.rowspan.extend_to_cover(pos.row);
            self.cells.insert(pos, cell);
        }
    }
}

impl TryFrom<&str> for Space {
    type Error = SpaceFromStringError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut space = Space::default();

        for (row, rowtext) in s.split('\n').enumerate() {
            for (col, c) in rowtext.chars().enumerate() {
                let pos = Position::try_new(col, row).unwrap();
                let cell = Cell::try_from(c).map_err(|ic| SpaceFromStringError(pos, ic))?;
                space.set(pos, cell);
            }
        }

        Ok(space)
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for col in &self.colspan {
            for row in &self.rowspan {
                let pos = Position::try_new(col, row).unwrap();
                write!(f, "{}", self.cells[&pos].glyph())?;
            }
        }

        Ok(())
    }
}
