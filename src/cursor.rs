use crate::geometry::Direction;
use crate::{Glyph, Widget};

use derive_new::new;

/// A [Cursor] defines a code execution context
#[derive(Debug, new)]
pub struct Cursor {
    /// The [Direction] the [Cursor] points in
    pub direction: Direction,
    /// A stack of values for the execution context
    #[new(default)]
    pub stack: Vec<i32>,
}

impl Cursor {
    /// Execute the given [Widget]
    ///
    /// # Note
    ///
    /// This is called after a cursor moves onto a cell, but prior
    /// to pushing `self` onto the [Cell](crate::Cell)'s cursor stack.
    pub fn execute(&mut self, widget: Widget) {
        use Direction::{North, South};
        use Widget::*;

        match widget {
            Noop => {}
            Dup => {
                if let Some(&i) = self.stack.last() {
                    self.stack.push(i);
                }
            }
            SwapN => {
                if let Some(mut ix) = self.stack.pop() {
                    let len = self.stack.len();

                    if ix < 0 {
                        ix += i32::try_from(len).unwrap();
                    }

                    let uix = usize::try_from(ix).unwrap();

                    if uix < len {
                        self.stack.swap(uix, len - 1);
                    }
                }
            }
            Ccw => {
                self.direction = self.direction.counterclockwise();
            }
            TurnIfZero => {
                if let Some(i) = self.stack.pop() {
                    if i != 0 {
                        self.direction = North;
                    }
                } else {
                    self.direction = South;
                }
            }
            Turn(dir) => {
                self.direction = dir;
            }
            PushDigit(digit) => {
                self.stack.push(i32::from(digit));
            }
            CalcArith(op) => {
                if let Some((b, a)) = self.stack.pop().zip(self.stack.pop()) {
                    self.stack.push(op.calc(a, b));
                }
            }
        };
    }
}

impl Glyph for Cursor {
    fn glyph(&self) -> char {
        use Direction::*;

        match self.direction {
            North => '▲',
            South => '▼',
            East => '▶',
            West => '◀',
        }
    }
}
