#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::fmt::Display;

use crate::errors::SpaceFromStringError;
use crate::geometry::{Position, Rect, Spanning as _};
use crate::{Cell, Cursor, Glyph as _, Physical, Widget};

#[derive(Debug)]
pub struct Space {
    cells: BTreeMap<Position, Cell>,
    span: Rect,
}

impl Space {
    fn insert<P>(&mut self, pos: Position, object: P)
    where
        P: Physical,
    {
        self.cells.entry(pos).or_default().insert(object);
        self.span.extend_to_cover(pos);
    }
}

impl Default for Space {
    fn default() -> Self {
        use crate::geometry::Direction::East;

        let mut s = Space {
            cells: BTreeMap::default(),
            span: Rect::default(),
        };

        s.insert(Position::new(0, 0), Cursor::new(East));
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
                let widget = Widget::try_from(c).map_err(|ic| SpaceFromStringError(pos, ic))?;
                assert!(space.cells.insert(pos, Cell::new(widget)).is_none());
            }
        }

        Ok(space)
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pos in &self.span {
            self.cells
                .get(&pos)
                .map(|c| c.glyph())
                .unwrap_or(' ')
                .fmt(f)?;
        }
        Ok(())
    }
}
