#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::fmt::Display;

use crate::errors::SpaceFromStringError;
use crate::geometry::{Position, Rect, Spanning as _};
use crate::{Cell, Cursor, Glyph as _};

#[derive(Debug)]
pub struct Space {
    /// Invariant: [Cell] is never stored as [Cell::Noop] because that's represented by absence
    cells: BTreeMap<Position, Cell>,
    cursors: BTreeMap<Position, Cursor>,
    span: Rect,
}

impl Space {
    fn set(&mut self, pos: Position, cell: Cell) {
        if matches!(cell, Cell::Noop) {
            self.span.extend_to_cover(pos);
            self.cells.insert(pos, cell);
        }
    }

    fn add_cursor(&mut self, pos: Position, cursor: Cursor) {
        self.span.extend_to_cover(pos);
        self.cursors.insert(pos, cursor);
    }
}

impl Default for Space {
    fn default() -> Self {
        use crate::geometry::Direction::East;

        let mut s = Space {
            cells: BTreeMap::default(),
            cursors: BTreeMap::default(),
            span: Rect::default(),
        };

        s.add_cursor(Position::new(0, 0), Cursor::new(East));
        s
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
        for pos in &self.span {
            self.cursors
                .get(&pos)
                .map(|c| c.glyph())
                .or(self.cells.get(&pos).map(|c| c.glyph()))
                .unwrap_or(' ')
                .fmt(f)?;
        }
        Ok(())
    }
}
