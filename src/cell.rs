mod physical;

use derive_new::new;

use crate::{Cursor, Glyph, Widget};

pub use self::physical::Physical;

/// A reference to a read-only default [Cell] value
pub const DEFAULT_CELL: &Cell = &Cell {
    widget: Widget::Noop,
    cursors: vec![],
};

/// A storage location which has a [Widget] and any number of [Cursor]s
#[derive(Debug, Default, new)]
pub struct Cell {
    widget: Widget,
    #[new(default)]
    cursors: Vec<Cursor>,
}

impl Cell {
    /// A [Cell] is empty when the [Widget] is [Noop](Widget::Noop) and there are no [Cursor]s there
    pub fn is_empty(&self) -> bool {
        matches!(self.widget, Widget::Noop) && self.cursors.is_empty()
    }

    /// Insert a physical value (either a [Widget] or [Cursor]) into this [Cell]
    pub fn insert<P>(&mut self, object: P)
    where
        P: Physical,
    {
        object.insert_into(self)
    }

    /// Remove the top [Cursor] if any
    pub fn pop_cursor(&mut self) -> Option<Cursor> {
        self.cursors.pop()
    }
}

impl Glyph for Cell {
    fn glyph(&self) -> char {
        self.cursors
            .last()
            .map(|c| c.glyph())
            .unwrap_or(self.widget.glyph())
    }
}
