use crossterm::cursor;
use ratatui::widgets;
use tui_input::{Input, InputRequest};

use std::env;

use std::collections::HashMap;
use std::path::PathBuf;

use super::{preview::Preview, file::FilePath};


/// Application.
pub struct Navigation {
    /// File manager
    pub working_dir: PathBuf,
    /// Cursor
    pub cursor: HashMap<PathBuf, usize>,

    pub preview: Preview,

    pub search_string: String,
}

impl Navigation {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let dir = env::current_dir().unwrap_or(PathBuf::from("/"));
        let mut current = dir.copy();
        let mut cursor = HashMap::new();

        while let Some(parent) = current.parent_dir() {
                let position = parent.search(current.name()).unwrap();
                cursor.insert(parent.copy(), position);
                current = parent;
        }

        Navigation {
            working_dir: dir,
            cursor: cursor,
            preview: Preview::new(),
            search_string: "".to_string(),
        }
    }

    pub fn selected(&self) -> Option<PathBuf> {
        let pos = match self.cursor.get(&self.working_dir) {
            Some(x) => *x,
            None => 0,
        }; 
        self.working_dir.children().unwrap().get(pos).map(|p| p.clone())
    }

    pub fn move_up(&mut self) {
        match self.working_dir.parent_dir() {
            Some(p) => self.working_dir = p,
            None => (),
        };
    }

    pub fn open(&mut self) {
        if let Some(file) = self.selected() {
            if file.is_dir() {
                self.working_dir = file;
            }
        }
    }

    pub fn update_search(&mut self, chr: char) {
        self.search_string.push(chr);
        match self.working_dir.search(self.search_string.clone()) {
            Some(x) => self.move_cursor_to(x),
            _ => self.search_string = chr.to_string(),
        }
    }

    fn get_cursor(&self) -> usize {
        match self.cursor.get(&self.working_dir) {
            None => 0,
            Some(x) => *x,
        }
    }

    fn move_cursor(&mut self, amount: i32) {
        let len = self.working_dir.children().unwrap().len() as i32;
        let new_value = if len != 0 {(self.get_cursor() as i32 + amount + len) % len} else {0};
        self.cursor.insert(self.working_dir.clone(), new_value as usize);
    }

    pub fn move_cursor_to(&mut self, position: usize) {
        self.move_cursor(position as i32 - self.get_cursor() as i32);
    }

    pub fn cursor_up(&mut self) {
        self.move_cursor(1);
    }

    pub fn cursor_down(&mut self) {
        self.move_cursor(-1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}