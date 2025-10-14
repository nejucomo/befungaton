mod physical;

use derive_new::new;

use crate::{Cursor, Glyph, Widget};

pub use self::physical::Physical;

#[derive(Debug, Default, new)]
pub struct Cell {
    widget: Widget,
    #[new(default)]
    cursors: Vec<Cursor>,
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        matches!(self.widget, Widget::Noop) && self.cursors.is_empty()
    }

    pub fn insert<P>(&mut self, object: P)
    where
        P: Physical,
    {
        object.insert_into(self)
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
