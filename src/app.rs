use crossterm::cursor;
use ratatui::widgets;
use tui_input::{Input, InputRequest};

use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};


use std::env;

use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf;
use std::process::Command;

use crate::files::file::{Contents, FilePath};


/// Application.
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// File manager
    pub working_dir: PathBuf,
    /// Cursor
    pub cursor: HashMap<PathBuf, usize>,
    // Content for third window
    /// Input state
    pub input_state: Input,
    pub contents: Contents,
    pub thread_pool: Vec<JoinHandle<()>>,
    pub sender: Sender<Contents>,
    pub receiver: Receiver<Contents>,
}

impl App {
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
        let (sender, receiver) = mpsc::channel();

        App {
            should_quit: false,
            working_dir: dir,
            cursor: cursor,
            input_state: "".into(),
            thread_pool: Vec::new(),
            contents: Contents::Other,
            sender, 
            receiver,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.refresh_contents();
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
        self.update_contents();
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

    pub fn get_state(&mut self, path: Option<PathBuf>) -> widgets::ListState {
        let cursor = path.and_then(|p| self.cursor.get(&p).copied());
        let cursor = cursor.unwrap_or(0);
        let mut state = widgets::ListState::default();
        state.select(Some(cursor));
        state
    }

    
    pub fn update_input(&mut self, chr: char) {
        let req = InputRequest::InsertChar(chr); 
        self.input_state.handle(req);
        let index = self.working_dir.search(self.get_input());
        match self.working_dir.search(self.get_input()) {
            Some(x) => self.move_cursor_to(x),
            None => {self.input_state.handle(InputRequest::DeleteLine);},
        };
    }
 
    fn update_contents(&mut self) {
        let (sender, receiver) = mpsc::channel::<Contents>();

        let selected_clone = self.selected();
        let sender_clone = sender.clone();

        self.sender = sender;
        self.receiver = receiver;
        


        let handle = thread::spawn(move || {
            if let Some(s) = selected_clone {
                sender_clone.send(s.contents()).unwrap();
            }

        });


        self.thread_pool.push(handle);
    }

    pub fn refresh_contents(&mut self) {
        match self.receiver.try_recv() {
            Ok(contents) => self.contents = contents,
            _ => (),
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