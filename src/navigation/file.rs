use std::path::PathBuf;
use std::env::consts;
use std::fs;
use mime_guess;

fn separator() -> String {
    (if consts::OS == "windows" {"\\"} else {"/"}).to_string()
}
#[derive(Clone)]
pub enum Contents {
    Children(Vec<PathBuf>),
    Text(String),
    Other,
}
pub trait FilePath {
    fn name(&self) -> String;

    fn path_name(&self) -> String;

    fn parent_dir(&self) -> Option<PathBuf>;

    fn children(&self) -> Option<Vec<PathBuf>>;

    fn contents(&self) -> Contents;

    fn search(&self, term: String) -> Option<usize>;

    fn is_text_file(&self) -> bool;

    fn copy(&self) -> Self;
}

impl FilePath for PathBuf {
    fn name(&self) -> String {
        let name = self.file_name().unwrap().to_str().unwrap().to_string();
        if self.is_dir() {
           return name + &separator();
        }
        name
    }

    fn path_name(&self) -> String {
        let path = self.to_str().unwrap().to_string().clone();
        match self.parent_dir() {
            None => path,
            _ => path + &separator(),
        }
    }

    fn parent_dir(&self) -> Option<PathBuf> {
        match self.parent() {
            None => None,
            Some(p) => Some(p.to_path_buf()),
        }
    }

    fn children(&self) -> Option<Vec<PathBuf>> {
        match self.read_dir() {
            Ok(entries) => Some(entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .collect()),
            Err(_) => None,
        }
    }

    fn contents(&self) -> Contents {
        if let Some(children) = self.children() {
            return Contents::Children(children);
        }
        if self.is_text_file() {
            return Contents::Text(fs::read_to_string(self.as_path()).unwrap_or("".to_string()))
        }
        Contents::Other
    }

    fn search(&self, term: String) -> Option<usize> {
        match self.children() {
            Some(children) => {
                children
                .iter()
                .position(|name| name.name().to_lowercase().starts_with(&term.to_lowercase()))    
            },
            _ => None,
        }
        
    }

    fn is_text_file(&self) -> bool {
        if let Some(name) = mime_guess::from_path(self.path_name()).first() {
            return name.type_().as_str() == "text"
        }
        false
    }

    fn copy(&self) -> Self {
        PathBuf::from(&self)
    }
}