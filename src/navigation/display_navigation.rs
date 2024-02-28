use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};


use std::env;

use std::collections::HashMap;
use std::path::PathBuf;

use crate::navigation::file::{Contents, FilePath};

use super::navigation::{self, Navigation};

#[derive(Clone)]
pub struct NavigationContent {
    pub name: String,
    pub path_name: String,
    pub working_dir: PathBuf,
    pub parent_dir: Option<PathBuf>,
    pub cursor: (usize, usize, usize),
    pub preview: Contents,
}

impl NavigationContent {
    fn new() -> Self {
        NavigationContent {
            name: "".to_string(),
            path_name: "".to_string(),
            working_dir: PathBuf::new(),
            parent_dir: None,
            cursor: (0, 0, 0),
            preview: Contents::Other,
        }
    }
}

/// Application.
pub struct DisplayNavigation {
    pub navigation_content: NavigationContent,
    pub thread_pool: Vec<JoinHandle<()>>,
    pub sender: Sender<NavigationContent>,
    pub receiver: Receiver<NavigationContent>,
}

impl DisplayNavigation {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();

        DisplayNavigation {
            navigation_content: NavigationContent::new(),
            thread_pool: Vec::new(),
            sender, 
            receiver,
        }
    }


    pub fn update_navigation(&mut self, navigation: &Navigation) {
        let (sender, receiver) = mpsc::channel::<NavigationContent>();

        let sender_clone = sender.clone();
        let working_dir = navigation.working_dir.clone();
        let cursor = navigation.cursor.clone();

        self.sender = sender;
        self.receiver = receiver;
        
        let handle = thread::spawn(move || {
            sender_clone.send(update_navigation_content(cursor, working_dir)).unwrap();
        });

        self.thread_pool.push(handle);
    }

    pub fn refresh_navigation(&mut self) {
        match self.receiver.try_recv() {
            Ok(contents) => self.navigation_content = contents,
            _ => (),
        }
    }
}

fn update_navigation_content(cursor_map: HashMap<PathBuf, usize>, working_dir: PathBuf) -> NavigationContent {
    let parent_dir = working_dir.parent_dir();
    let mut cursor = (0, 0, 0);
    let preview = working_dir.contents();

    cursor.0 = parent_dir.as_ref().map_or(0, |p| cursor_map.get(p).unwrap_or(&0).clone());
    cursor.1 = cursor_map.get(&working_dir).unwrap().clone();
    if let Contents::Children(children) = &preview {
        cursor.2 = children.get(cursor.1).map_or(0, |p| cursor_map.get(p).unwrap_or(&0).clone())
    } 

    NavigationContent {
        name: working_dir.name(),
        path_name: working_dir.path_name(),
        working_dir,
        parent_dir,
        cursor,
        preview,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}