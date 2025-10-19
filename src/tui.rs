use std::io::Write as _;

use crossterm::{QueueableCommand as _, terminal};

use crate::Space;
use crate::errors::IOParseError;

/// A Terminal User Interface
#[derive(Debug)]
pub struct Tui {
    space: Space,
    stdout: std::io::Stdout,
}

impl Default for Tui {
    fn default() -> Self {
        Self {
            space: Space::default(),
            stdout: std::io::stdout(),
        }
    }
}

impl Tui {
    /// Load the given path into the [Space]
    pub fn load<P>(&mut self, path: P) -> Result<(), IOParseError>
    where
        P: AsRef<std::path::Path>,
    {
        self.space = Space::try_from(path.as_ref())?;
        Ok(())
    }

    /// Take control of the terminal and launch the UI.
    pub fn ui_loop(self) -> std::io::Result<()> {
        in_raw_mode(|| self.raw_ui_loop())
    }

    fn raw_ui_loop(mut self) -> std::io::Result<()> {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?;

        self.stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(400));
        Ok(())
    }
}

fn in_raw_mode<F>(f: F) -> std::io::Result<()>
where
    F: FnOnce() -> std::io::Result<()>,
{
    terminal::enable_raw_mode()?;
    let res = f();
    terminal::disable_raw_mode()?;
    res
}
