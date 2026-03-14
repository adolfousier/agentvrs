use crossterm::event::{self, Event, KeyEvent};
use std::time::Duration;

pub enum TuiEvent {
    Key(KeyEvent),
    Tick,
    Resize(u16, u16),
}

pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(tick_rate_ms: u64) -> Self {
        Self {
            tick_rate: Duration::from_millis(tick_rate_ms),
        }
    }

    pub fn next(&self) -> anyhow::Result<TuiEvent> {
        if event::poll(self.tick_rate)? {
            match event::read()? {
                Event::Key(key) => Ok(TuiEvent::Key(key)),
                Event::Resize(w, h) => Ok(TuiEvent::Resize(w, h)),
                _ => Ok(TuiEvent::Tick),
            }
        } else {
            Ok(TuiEvent::Tick)
        }
    }
}
