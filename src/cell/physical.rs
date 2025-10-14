use crate::{Cell, Cursor, Widget};

/// Values which can be stored in cells
pub trait Physical {
    fn insert_into(self, cell: &mut Cell);
}

impl Physical for Widget {
    fn insert_into(self, cell: &mut crate::Cell) {
        cell.widget = self;
    }
}

impl Physical for Cursor {
    fn insert_into(self, cell: &mut Cell) {
        cell.cursors.push(self);
    }
}
