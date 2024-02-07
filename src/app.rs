use ratatui::widgets;
use tui_input::{Input, InputRequest};

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use crate::files::paths::FolderPath;
use crate::files::cursor::Cursor;

/// Application.
#[derive(Debug)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// The current path
    pub path: PathBuf,
    /// The cursor for all folders
    pub cursor: HashMap<PathBuf, usize>,
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
            path: PathBuf::from("/home/frolle"),
            cursor: HashMap::new(),
            input_state: "".into(),
            state: widgets::ListState::default(), 
        }
    }
    //fs::read_dir("~").unwrap()
    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    pub fn get_state(&mut self, path: PathBuf) -> &mut widgets::ListState {
        let cursor = self.cursor.get_cursor(&path);
        self.state.select(Some(cursor));
        &mut self.state
    }

    pub fn selected(&self) -> PathBuf {
        self.path.child(self.cursor.get_cursor(&self.path))
    }
    /// Moves up one folder in the hierarchy
    pub fn move_up(&mut self) {
        self.path = self.path.parent_folder();
    }

    pub fn move_down(&mut self) {
        let child = self.path.child(self.cursor.get_cursor(&self.path));
        if child.is_dir() {
            self.path = child;
            return
        }
    }

    /// Handles the position of the cursor
    pub fn cursor_up(&mut self) {
        self.cursor.move_cursor(&self.path, self.cursor.get_cursor(&self.path) + 1);
    }

    pub fn cursor_down(&mut self) {
        let new_position = self.cursor.get_cursor(&self.path) + self.path.children().len() - 1;
        self.cursor.move_cursor(&self.path, new_position);
    }

    pub fn update_input(&mut self, chr: char) {
        let req = InputRequest::InsertChar(chr); 
        self.input_state.handle(req);
        let index = self.path.search(self.get_input());
        if index == -1 {
            self.input_state.handle(InputRequest::DeleteLine);
        } else {
            self.cursor.move_cursor(&self.path, index as usize);
        }
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