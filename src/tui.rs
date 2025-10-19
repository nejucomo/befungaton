use crossterm::{ExecutableCommand as _, terminal};

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
    pub fn ui_loop(mut self) -> std::io::Result<()> {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;

        std::thread::sleep(std::time::Duration::from_millis(1000));
        Ok(())
    }
}
