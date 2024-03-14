use std::char;

use ratatui::widgets;
use tui_input::{Input, InputRequest};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::navigation::{self, file::{Contents, FilePath}, navigation::Navigation};

/// Application.
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    
    pub navigation: Navigation,

}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let mut navigation = Navigation::new();
        navigation.preview.update(navigation.selected());

        App {
            should_quit: false,
            navigation,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.navigation.preview.refresh();
    }

    pub fn action(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.quit(),
            KeyCode::Left  => self.navigation.move_up(),
            KeyCode::Right => self.navigation.open(),
            KeyCode::Down  => self.navigation.cursor_up(),
            KeyCode::Up    => self.navigation.cursor_down(),
            KeyCode::Char(chr) if chr.is_uppercase() => {self.navigation.update_search(chr.to_ascii_lowercase())},
            _ => {}
        };
        self.navigation.preview.preview = Contents::Other;
        self.navigation.preview.update(self.navigation.selected());
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}