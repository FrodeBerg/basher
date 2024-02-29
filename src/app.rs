use ratatui::widgets;
use tui_input::{Input, InputRequest};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::navigation::{self, file::Contents, navigation::Navigation};

enum Action {
    LowercaseKey(char),
}
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
            //KeyCode::Char(chr) => {app.navigation.update_input(chr)},
            _ => {}
        };
        self.navigation.preview.preview = Contents::Other;
        self.navigation.preview.update(self.navigation.selected());
    }

    /*
    pub fn update_input(&mut self, chr: char) {
        let req = InputRequest::InsertChar(chr); 
        self.input_state.handle(req);
        let index = self.working_dir.search(self.get_input());
        match self.working_dir.search(self.get_input()) {
            Some(x) => self.move_cursor_to(x),
            None => {self.input_state.handle(InputRequest::DeleteLine);},
        };
    }
     */

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}