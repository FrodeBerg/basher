use std::collections::HashMap;
use std::path::PathBuf;

use crate::files::paths::FolderPath;

pub trait Cursor {
    fn get_cursor(&self, path: &PathBuf) -> usize; 

    fn move_cursor(&mut self, path: &PathBuf, amount: usize); 
}

impl Cursor for HashMap<PathBuf, usize> {
    fn get_cursor(&self, path: &PathBuf) -> usize {
        match self.get(path) {
            None => 0,
            Some(x) => *x,
        }
    }

    fn move_cursor(&mut self, path: &PathBuf, position: usize) {
        self.insert(path.clone(), position % path.children().len()); 
    }
}