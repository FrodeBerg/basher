use std::collections::HashMap;
use std::path::PathBuf;
use std::env;
use crate::files::file::{Folder, FilePath};

use super::file::Type;

#[derive(Debug)]
pub struct FileManager {
    pub cursor: HashMap<PathBuf, usize>,
    pub folder: Folder,
}


impl FileManager {
    pub fn working_dir() -> Self {
        let dir = env::current_dir().unwrap();
        let mut current = Folder::from_path(&dir);
        let mut cursor = HashMap::new();

        while let Some(parent) = current.parent_folder() {
                let position = parent.search(current.name()).unwrap();
                cursor.insert(parent.path.copy(), position);
                current = parent;
        }

        FileManager {
            cursor: cursor,
            folder: Folder{path: dir},
        }
    }

    pub fn selected(&self) -> Option<PathBuf> {
        let pos = match self.cursor.get(&self.folder.path) {
            Some(x) => *x,
            None => 0,
        }; 
        self.folder.children().get(pos).map(|p| p.clone())
    }

    pub fn move_up(&mut self) {
        match self.folder.parent_folder() {
            Some(p) => self.folder = p,
            None => (),
        };
    }

    pub fn open(&mut self) {
        if let Some(file) = self.selected() {
            match file.file_type() {
                Type::Folder(folder) => self.folder = folder,
                _ => (),
            };
        }
    }

    fn get_cursor(&self) -> usize {
        match self.cursor.get(&self.folder.path) {
            None => 0,
            Some(x) => *x,
        }
    }

    fn move_cursor(&mut self, amount: i32) {
        let len = self.folder.children().len() as i32;
        let new_value = self.get_cursor() as i32 + amount;
        self.cursor.insert(self.folder.path.clone(), ((new_value + len) % len) as usize);
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