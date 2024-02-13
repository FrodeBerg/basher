use crossterm::cursor;
use ratatui::widgets;
use tui_input::{Input, InputRequest};

use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf;
use std::process::Command;

use crate::files::file_manager::FileManager;

/// Application.
#[derive(Debug)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// File manager
    pub file_manager: FileManager,
    /// The state 
    pub state: widgets::ListState,
    /// Input state
    pub input_state: Input,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        App {
            should_quit: false,
            file_manager: FileManager::working_dir(),
            input_state: "".into(),
            state: widgets::ListState::default(), 
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    pub fn get_state(&mut self, path: Option<PathBuf>) -> &mut widgets::ListState {
        let cursor = path.and_then(|p| self.file_manager.cursor.get(&p).copied());
        let cursor = cursor.unwrap_or(0);
        self.state.select(Some(cursor));
        &mut self.state
    }

    
    pub fn update_input(&mut self, chr: char) {
        let req = InputRequest::InsertChar(chr); 
        self.input_state.handle(req);
        let index = self.file_manager.folder.search(self.get_input());
        match self.file_manager.folder.search(self.get_input()) {
            Some(x) => self.file_manager.move_cursor_to(x),
            None => {self.input_state.handle(InputRequest::DeleteLine);},
        };
    }
 
    pub fn get_input(&self) -> String {
        self.input_state.to_string()
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