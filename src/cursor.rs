use crate::{Cell, Direction};

use derive_new::new;

#[derive(Debug, new)]
pub struct CursorState {
    pub direction: Direction,
    #[new(default)]
    pub stack: Vec<Cell>,
}
