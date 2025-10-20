use std::io::{Result, Stdout, Write as _, stdout};

use crossterm::{cursor, style, terminal};

use crate::Space;
use crate::errors::IOParseError;

/// A Terminal User Interface
#[derive(Debug)]
pub struct Tui {
    space: Space,
    stdout: Stdout,
}

impl Default for Tui {
    fn default() -> Self {
        Self {
            space: Space::default(),
            stdout: stdout(),
        }
    }
}

impl Tui {
    /// Load the given path into the [Space]
    pub fn load<P>(&mut self, path: P) -> std::result::Result<(), IOParseError>
    where
        P: AsRef<std::path::Path>,
    {
        self.space = Space::try_from(path.as_ref())?;
        Ok(())
    }

    /// Take control of the terminal and launch the UI.
    pub fn ui_loop(self) -> Result<()> {
        in_raw_mode(|| self.raw_ui_loop())
    }

    fn raw_ui_loop(mut self) -> Result<()> {
        self.redraw()?;
        self.stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(3000));
        Ok(())
    }

    fn redraw(&mut self) -> Result<()> {
        // ╒╕╘╛│═
        const TITLE: &str = "🤖BEFUNGATON🤖";
        const TITLE_LEN: u16 = 12; // Character count

        let (cols, rows) = terminal::size()?;

        crossterm::queue!(
            self.stdout,
            terminal::Clear(terminal::ClearType::All),
            style::SetBackgroundColor(style::Color::Black),
            style::SetForegroundColor(style::Color::Magenta),
            cursor::MoveTo((cols - TITLE_LEN) / 2, 0),
            style::Print(TITLE),
            style::SetBackgroundColor(style::Color::Black),
            style::SetForegroundColor(style::Color::DarkGrey),
            cursor::MoveTo(0, 1),
            style::Print("╒"),
            cursor::MoveTo(cols - 1, 1),
            style::Print("╕"),
            cursor::MoveTo(0, rows - 1),
            style::Print("╘"),
            cursor::MoveTo(cols - 1, rows - 1),
            style::Print("╛"),
        )?;

        for row in [1, rows - 1] {
            for col in 1..cols - 1 {
                crossterm::queue!(self.stdout, cursor::MoveTo(col, row), style::Print("═"))?;
            }
        }

        for row in 2..rows - 1 {
            for col in [0, cols - 1] {
                crossterm::queue!(self.stdout, cursor::MoveTo(col, row), style::Print("│"))?;
            }
        }

        Ok(())
    }
}

fn in_raw_mode<F>(f: F) -> Result<()>
where
    F: FnOnce() -> Result<()>,
{
    terminal::enable_raw_mode()?;
    let res = f();
    terminal::disable_raw_mode()?;
    res
}
