use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::App;

pub fn handle_events(app: &mut App) -> anyhow::Result<()> {
    if event::poll(Duration::from_millis(16)).unwrap_or(false) {
        let e = event::read()?;

        match e {
            Event::Key(KeyEvent {
                kind: KeyEventKind::Press,
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                ..
            }) => app.insert_char(c),

            Event::Key(KeyEvent {
                kind: KeyEventKind::Press,
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => app.quit(),

            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => app.quit(),

            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                ..
            }) => app.get_manga_list(),

            Event::Key(KeyEvent {
                code: KeyCode::Left,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
                ..
            }) => app.move_cursor_left(),

            Event::Key(KeyEvent {
                code: KeyCode::Right,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
                ..
            }) => app.move_cursor_right(),

            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
                ..
            }) => app.delete_current_char(),

            _ => {}
        }
    }
    Ok(())
}
