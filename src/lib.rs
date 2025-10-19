//! `befungaton` is a [befunge](https://esolangs.org/wiki/Befunge)-like interpreter
#![feature(impl_trait_in_assoc_type)]
#![deny(missing_docs, unsafe_code)]

mod cell;
pub mod cli;
mod cursor;
pub mod errors;
pub mod geometry;
mod glyph;
mod space;
mod tui;
mod widget;

pub use self::cell::{Cell, DEFAULT_CELL, Physical};
pub use self::cursor::Cursor;
pub use self::glyph::Glyph;
pub use self::space::Space;
pub use self::tui::Tui;
pub use self::widget::Widget;
