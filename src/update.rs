use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    if key_event.modifiers == KeyModifiers::CONTROL {
        match  key_event.code {
            KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Char('q') => {
                app.quit()
            },
            KeyCode::Char('h') => app.move_up(),
            KeyCode::Char('l') => app.move_down(),
            KeyCode::Char('j') => app.cursor_up(),
            KeyCode::Char('k') => app.cursor_down(),
            _ => {},
        }
        return;
    }
    match key_event.code {
        KeyCode::Esc => app.quit(),
        KeyCode::Left  => app.move_up(),
        KeyCode::Right => app.move_down(),
        KeyCode::Up  => app.cursor_up(),
        KeyCode::Down    => app.cursor_down(),
        //KeyCode::Char(chr) => {app.update_input(chr)},
        _ => {}
    };
}