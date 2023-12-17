use std::time::Duration;

use crossterm::event::{self, Event, KeyEventKind, KeyCode, KeyModifiers};

use crate::app::App;

pub fn handle_events(app: &mut App) -> anyhow::Result<()> {
    if event::poll(Duration::from_millis(16)).unwrap_or(false) {
        if let Event::Key(e) = event::read()? {
            if e.kind == KeyEventKind::Press {
                match e.code {
                    KeyCode::Char('c') if e.modifiers == KeyModifiers::CONTROL => {
                        app.quit();
                    },
                    _ => {},
                }
            }
        }
    }
    Ok(())
}
