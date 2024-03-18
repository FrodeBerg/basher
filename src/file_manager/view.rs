use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};

use std::path::PathBuf;

use crate::file_manager::file::{Contents, FilePath};

use super::file;
/// Application.
pub struct View {
    pub tx: Sender<Contents>,
    pub rx: Receiver<Contents>,
    pub preview: Contents,
    pub working: Vec<PathBuf>,
    pub parent: Vec<PathBuf>,
}

impl View {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        View {
            tx, 
            rx,
            preview: Contents::Other,
            working: Vec::new(),
            parent: Vec::new(),
        }
    }

    pub fn update(&mut self, selected_dir: Option<PathBuf>) {
        let (tx, rx) = mpsc::channel();
        let tx_clone = tx.clone();

        self.tx = tx;
        self.rx = rx;


        let selected_dir_clone = selected_dir.clone();
        thread::spawn(move || {
            let preview = selected_dir_clone.map_or(Contents::Other, |dir| dir.contents());
            match preview {
                Contents::Text(text) => {
                    tx_clone.send(Contents::Text(text)).unwrap_or(())
                },
                _ => tx_clone.send(preview).unwrap_or(()),
            }    
        });

    }

    pub fn refresh(&mut self) {
        match self.rx.try_recv() {
            Ok(preview) => self.preview = preview,
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}