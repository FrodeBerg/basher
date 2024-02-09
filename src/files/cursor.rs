use std::collections::HashMap;
use std::path::PathBuf;

use crate::files::paths::FolderPath;

pub trait Cursor {
    fn get_cursor(&self, path: &PathBuf) -> usize; 

    fn move_cursor(&mut self, path: &PathBuf, amount: i32); 

    fn move_cursor_to(&mut self, path: &PathBuf, amount: usize);
}

impl Cursor for HashMap<PathBuf, usize> {
    fn get_cursor(&self, path: &PathBuf) -> usize {
        match self.get(path) {
            None => 0,
            Some(x) => *x,
        }
    }

    fn move_cursor(&mut self, path: &PathBuf, amount: i32) {
        let len = path.children().len() as i32;
        let new_value = self.get_cursor(path) as i32 + amount;
        self.insert(path.clone(), ((new_value + len) % len) as usize);
    }

    fn move_cursor_to(&mut self, path: &PathBuf, position: usize) {
        self.move_cursor(path, position as i32 - self.get_cursor(path) as i32);
    }
}