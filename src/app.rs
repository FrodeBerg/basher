use ratatui::widgets;
use tui_input::{Input, InputRequest};

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use crate::files::files::{File, Folder};

/// Application.
#[derive(Debug)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// The current folder
    pub folder: Box<Folder>,
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
            folder: Box::new(Folder::root()),
            input_state: "".into(),
            state: widgets::ListState::default(), 
        }
    }
    //fs::read_dir("~").unwrap()
    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    pub fn get_state(&mut self, pos: Option<usize>) -> &mut widgets::ListState {
        self.state.select(pos);
        &mut self.state
    }

    /// Moves up one folder in the hierarchy
    pub fn move_up(&mut self) {
        if let Some(parent) = self.folder.parent.take() {
            self.folder = parent;
        }
    }

    pub fn move_down(&mut self) {
        match self.folder.selected() {
            File::Folder(mut folder) => {
                folder.parent = Some(self.folder.clone());
                self.folder = Box::new(folder);
            },
        }
    }

    /// Handles the position of the cursor
    pub fn move_cursor(&mut self, amount: i32) {
        let len = self.folder.children.len() as i32;
        self.folder.cursor = ((self.folder.cursor as i32 + amount + len) % len) as usize;
    }
    pub fn cursor_up(&mut self) {
        self.move_cursor(-1);
    }

    pub fn cursor_down(&mut self) {
        self.move_cursor(1);
    }
    
    pub fn update_input(&mut self, chr: char) {
        let req = InputRequest::InsertChar(chr); 
        self.input_state.handle(req);
        if !self.folder.search(self.get_input()) {
            self.input_state.handle(InputRequest::DeleteLine);
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