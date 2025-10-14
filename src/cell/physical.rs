use crate::{Cell, Cursor, Widget};

/// Values which can be stored in cells
pub trait Physical {
    /// Insert the physical item into the [Cell]
    fn insert_into(self, cell: &mut Cell);
}

impl Physical for Widget {
    fn insert_into(self, cell: &mut crate::Cell) {
        cell.widget = self;
    }
}

impl Physical for Cursor {
    fn insert_into(mut self, cell: &mut Cell) {
        use Widget::*;

        match cell.widget {
            Noop => {}
            Turn(d) => {
                self.direction = d;
            }
        };

        cell.cursors.push(self);
    }
}
