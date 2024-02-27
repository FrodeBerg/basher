use std::collections::HashMap;
use std::path::PathBuf;
use std::env;
use crate::files::file::{Directory, FilePath};

use super::file::Type;

#[derive(Debug)]
pub struct FileManager {
    pub cursor: HashMap<PathBuf, usize>,
    pub working_dir: Directory,
}


impl FileManager {
    pub fn working_dir() -> Self {
        let dir = env::current_dir().unwrap();
        let mut current = Directory::from_path(&dir);
        let mut cursor = HashMap::new();

        while let Some(parent) = current.path.parent_dir() {
                let position = parent.search(current.path.name()).unwrap();
                cursor.insert(parent.path.copy(), position);
                current = parent;
        }

        FileManager {
            cursor: cursor,
            working_dir: Directory{path: dir},
        }
    }

    pub fn selected(&self) -> Option<PathBuf> {
        let pos = match self.cursor.get(&self.working_dir.path) {
            Some(x) => *x,
            None => 0,
        }; 
        self.working_dir.children().get(pos).map(|p| p.clone())
    }

    pub fn move_up(&mut self) {
        match self.working_dir.path.parent_dir() {
            Some(p) => self.working_dir = p,
            None => (),
        };
    }

    pub fn open(&mut self) {
        if let Some(file) = self.selected() {
            match file.file_type() {
                Type::Directory(dir) => self.working_dir = dir,
                _ => (),
            };
        }
    }

    fn get_cursor(&self) -> usize {
        match self.cursor.get(&self.working_dir.path) {
            None => 0,
            Some(x) => *x,
        }
    }

    fn move_cursor(&mut self, amount: i32) {
        let len = self.working_dir.children().len() as i32;
        let new_value = if len != 0 {(self.get_cursor() as i32 + amount + len) % len} else {0};
        self.cursor.insert(self.working_dir.path.clone(), new_value as usize);
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