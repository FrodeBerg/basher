use std::path::PathBuf;
use std::env::consts;
use std::fs;

fn separator() -> String {
    (if consts::OS == "windows" {"\\"} else {"/"}).to_string()
}
pub enum Type {
    Folder(Folder),
    TextFile(TextFile),
}
#[derive(Debug, Clone)]
pub struct Folder {
    pub path: PathBuf,
}

impl Folder {

    pub fn from_path(path: &PathBuf) -> Self {
        Folder{path:path.clone()}
    }

    pub fn name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string() + &separator()
    }

    pub fn path_name(&self) -> String {
        let path = self.path.to_str().unwrap().to_string().clone();
        match self.parent_folder() {
            None => path,
            _ => path + &separator(),
        }
    }

    pub fn parent_folder(&self) -> Option<Folder> {
        match self.path.parent() {
            None => None,
            Some(p) => Some(Folder{path: p.to_path_buf()}),
        }
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
    fn file_type(&self) -> Type;

    fn name(&self) -> String;

    fn copy(&self) -> Self;
}

impl FilePath for PathBuf {
    fn name(&self) -> String {
        match self.file_type() {
            Type::Folder(folder) => folder.name(),
            _ => self.file_name().unwrap().to_str().unwrap().to_string(),
        }
    }

    fn file_type(&self) -> Type {
        if self.is_dir() {
            return Type::Folder(Folder{path:self.clone()});
        }  
        Type::TextFile(TextFile{path:self.clone()})
    }

    fn copy(&self) -> Self {
        PathBuf::from(&self)
    }
}