//! Terminal event iteration

use std::time::{Duration, Instant};

#[derive(Debug)]
pub(super) enum Event {
    Tick,
    Crossterm(crossterm::event::Event),
}

pub(super) fn iterator() -> impl Iterator<Item = std::io::Result<Event>> {
    EventIterator::default()
}

/// Iterate over Events
#[derive(Debug)]
pub(super) struct EventIterator {
    last_tick: Option<Instant>,
    tick_length: Duration,
}

impl EventIterator {
    fn tick_remaining(&self) -> Duration {
        self.last_tick
            .map(|lt| std::cmp::min(self.tick_length, Instant::now() - lt))
            .unwrap_or(self.tick_length)
    }

    fn next_event(&mut self) -> std::io::Result<Event> {
        use Event::*;
        use crossterm::event;

        if event::poll(self.tick_remaining())? {
            event::read().map(Crossterm)
        } else {
            self.last_tick = Some(Instant::now());
            Ok(Tick)
        }
    }
}

impl Default for EventIterator {
    fn default() -> Self {
        Self {
            last_tick: None,
            tick_length: Duration::from_millis(1000),
        }
    }
}

impl Iterator for EventIterator {
    type Item = std::io::Result<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_event())
    }
}
