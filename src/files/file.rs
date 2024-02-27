use std::path::PathBuf;
use std::env::consts;
use std::fs;
use mime_guess;

fn separator() -> String {
    (if consts::OS == "windows" {"\\"} else {"/"}).to_string()
}
pub enum Type {
    Directory(Directory),
    TextFile(TextFile),
    Other,
}
#[derive(Debug, Clone)]
pub struct Directory {
    pub path: PathBuf,
}

impl Directory {

    pub fn from_path(path: &PathBuf) -> Self {
        Directory{path:path.clone()}
    }

    pub fn children(&self) -> Vec<PathBuf> {
        let mut files: Vec<PathBuf> = match self.path.read_dir() {
            Ok(entries) => entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .collect(),
            Err(_) => Vec::new(),
        };
        files.sort_by(|a, b| {
            a.name().cmp(&b.name())
        });
        files
    }

    pub fn search(&self, term: String) -> Option<usize> {
        self
            .children()
            .iter()
            .position(|name| name.name().to_lowercase().starts_with(&term.to_lowercase()))
    }
}
#[derive(Debug)]
pub struct TextFile {
    path: PathBuf,
}

impl TextFile {
    pub fn read(&self) -> Option<String> {
        fs::read_to_string(self.path.as_path()).ok()
    }
}

pub trait FilePath {
    fn name(&self) -> String;

    fn path_name(&self) -> String;

    fn parent_dir(&self) -> Option<Directory>;

    fn file_type(&self) -> Type;

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

    fn parent_dir(&self) -> Option<Directory> {
        match self.parent() {
            None => None,
            Some(p) => Some(Directory{path: p.to_path_buf()}),
        }
    }

    fn file_type(&self) -> Type {
        if self.is_dir() {
            return Type::Directory(Directory{path:self.clone()});
        }  
        if self.is_text_file() {
            return Type::TextFile(TextFile{path:self.clone()});
        }
        Type::Other
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