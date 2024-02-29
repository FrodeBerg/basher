use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};

use std::path::PathBuf;

use crate::navigation::file::{Contents, FilePath};

use super::navigation::Navigation;

/// Application.
pub struct Preview {
    pub thread_pool: Vec<JoinHandle<()>>,
    pub tx: Sender<Contents>,
    pub rx: Receiver<Contents>,
    pub preview: Contents,
}

impl Preview {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        Preview {
            thread_pool: Vec::new(),
            tx, 
            rx,
            preview: Contents::Other,
        }
    }

    pub fn update(&mut self, working_dir: PathBuf) {
        let (tx, rx) = mpsc::channel::<Contents>();
        let tx_clone = tx.clone();

        self.tx = tx;
        self.rx = rx;
        
        let handle = thread::spawn(move || {
            tx_clone.send(working_dir.contents()).unwrap();
        });

        self.thread_pool.push(handle);
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