use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    if key_event.modifiers == KeyModifiers::CONTROL {
        match  key_event.code {
            KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Char('q') => {
                app.quit()
            },
            KeyCode::Char('h') => app.file_manager.move_up(),
            KeyCode::Char('l') => app.file_manager.open(),
            KeyCode::Char('j') => app.file_manager.cursor_up(),
            KeyCode::Char('k') => app.file_manager.cursor_down(),
            _ => {},
        }
        return;
    }
    match key_event.code {
        KeyCode::Esc => app.quit(),

        KeyCode::Left  => app.file_manager.move_up(),
        KeyCode::Right => app.file_manager.open(),
        KeyCode::Down  => app.file_manager.cursor_up(),
        KeyCode::Up    => app.file_manager.cursor_down(),
        KeyCode::Char(chr) => {app.update_input(chr)},
        _ => {}
    };
}