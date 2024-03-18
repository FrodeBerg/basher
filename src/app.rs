use std::char;

use ratatui::widgets;
use tui_input::{Input, InputRequest};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::file_manager::{self, file::{Contents, FilePath}, file_manager::FileManager};

/// Application.
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    
    pub file_manager: FileManager,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let mut file_manager = FileManager::new();
        file_manager.view.update(file_manager.selected());

        App {
            should_quit: false,
            file_manager,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.file_manager.view.refresh();
    }

    pub fn action(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.quit(),
            KeyCode::Left  => self.file_manager.move_up(),
            KeyCode::Right | KeyCode::Tab => self.file_manager.open(),
            KeyCode::Down  => self.file_manager.cursor_up(),
            KeyCode::Up    => self.file_manager.cursor_down(),
            KeyCode::Char(chr) if chr.is_uppercase() => {self.file_manager.update_search(chr.to_ascii_lowercase())},
            _ => {}
        };
        self.file_manager.view.preview = Contents::Other;
        self.file_manager.view.update(self.file_manager.selected());
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