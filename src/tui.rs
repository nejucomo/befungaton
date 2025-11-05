//! An interactive terminal ui
mod event;

use std::io::Result;

use crossterm::event::KeyEvent;
use derive_new::new;
use ratatui::DefaultTerminal;
use ratatui::widgets::Block;

use crate::Space;

use self::event::Event;

/// Run the interactive tui with the given space
pub fn run(space: Space) -> Result<()> {
    let term = ratatui::init();
    let r = Tui::new(space, term).run();
    ratatui::restore();
    r
}

/// A Terminal User Interface
#[derive(Debug, new)]
pub struct Tui {
    #[allow(dead_code)]
    space: Space,
    term: DefaultTerminal,
}

impl Tui {
    fn run(mut self) -> Result<()> {
        self.redraw()?;

        for evres in event::iterator() {
            let ev = evres?;
            self.handle_event(ev)?;
        }
        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Result<()> {
        use Event::*;
        use crossterm::event::Event::*;

        match event {
            Tick => Err(std::io::Error::other("todo")),
            // Crossterm(FocusGained) => Err(std::io::Error::other("todo")),
            // Crossterm(FocusLost) => Err(std::io::Error::other("todo")),
            Crossterm(Key(key_event)) => self.handle_key(key_event),
            // Crossterm(Mouse(mouse_event)) => Err(std::io::Error::other("todo")),
            // Crossterm(Paste(_)) => Err(std::io::Error::other("todo")),
            // Crossterm(Resize(_, _)) => self.redraw(),
            Crossterm(other) => Err(std::io::Error::other(format!("handle {other:?}"))),
        }
    }

    fn handle_key(&mut self, keyev: KeyEvent) -> Result<()> {
        use crossterm::event::KeyCode::*;

        match keyev.code {
            // Backspace => Err(std::io::Error::other("todo")),
            // Enter => Err(std::io::Error::other("todo")),
            // Left => Err(std::io::Error::other("todo")),
            // Right => Err(std::io::Error::other("todo")),
            // Up => Err(std::io::Error::other("todo")),
            // Down => Err(std::io::Error::other("todo")),
            // Home => Err(std::io::Error::other("todo")),
            // End => Err(std::io::Error::other("todo")),
            // PageUp => Err(std::io::Error::other("todo")),
            // PageDown => Err(std::io::Error::other("todo")),
            // Tab => Err(std::io::Error::other("todo")),
            // BackTab => Err(std::io::Error::other("todo")),
            // Delete => Err(std::io::Error::other("todo")),
            // Insert => Err(std::io::Error::other("todo")),
            // F(_) => Err(std::io::Error::other("todo")),
            Char(c) => Err(std::io::Error::other(format!("c: {c:?}"))),
            // Null => Err(std::io::Error::other("todo")),
            // Esc => Err(std::io::Error::other("todo")),
            // CapsLock => Err(std::io::Error::other("todo")),
            // ScrollLock => Err(std::io::Error::other("todo")),
            // NumLock => Err(std::io::Error::other("todo")),
            // PrintScreen => Err(std::io::Error::other("todo")),
            // Pause => Err(std::io::Error::other("todo")),
            // Menu => Err(std::io::Error::other("todo")),
            // KeypadBegin => Err(std::io::Error::other("todo")),
            // Media(media_key_code) => Err(std::io::Error::other("todo")),
            // Modifier(modifier_key_code) => Err(std::io::Error::other("todo")),
            other => Err(std::io::Error::other(format!(
                "implement key handling for {other:?}"
            ))),
        }
    }

    fn redraw(&mut self) -> Result<()> {
        self.term.draw(|frame| {
            frame.render_widget(Block::bordered().title("Title Bar"), frame.area());
        })?;

        // // ╒╕╘╛│═
        // const TITLE: &str = "🤖BEFUNGATON🤖";
        // const TITLE_LEN: u16 = 12; // Character count

        // let (cols, rows) = terminal::size()?;

        // crossterm::queue!(
        //     self.stdout,
        //     terminal::Clear(terminal::ClearType::All),
        //     style::SetBackgroundColor(style::Color::Black),
        //     style::SetForegroundColor(style::Color::Magenta),
        //     cursor::MoveTo((cols - TITLE_LEN) / 2, 0),
        //     style::Print(TITLE),
        //     style::SetBackgroundColor(style::Color::Black),
        //     style::SetForegroundColor(style::Color::DarkGrey),
        //     cursor::MoveTo(0, 1),
        //     style::Print("╒"),
        //     cursor::MoveTo(cols - 1, 1),
        //     style::Print("╕"),
        //     cursor::MoveTo(0, rows - 1),
        //     style::Print("╘"),
        //     cursor::MoveTo(cols - 1, rows - 1),
        //     style::Print("╛"),
        // )?;

        // for row in [1, rows - 1] {
        //     for col in 1..cols - 1 {
        //         crossterm::queue!(self.stdout, cursor::MoveTo(col, row), style::Print("═"))?;
        //     }
        // }

        // for row in 2..rows - 1 {
        //     for col in [0, cols - 1] {
        //         crossterm::queue!(self.stdout, cursor::MoveTo(col, row), style::Print("│"))?;
        //     }
        // }

        Ok(())
    }
}
