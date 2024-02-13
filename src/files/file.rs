use std::path::PathBuf;
use std::env::consts;

fn separator() -> String {
    (if consts::OS == "windows" {"\\"} else {"/"}).to_string()
}
pub enum Type {
    Folder(Folder),
    TextFile(TextFile),
}
#[derive(Debug)]
pub struct Folder {
    pub path: PathBuf,
}

impl Folder {

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
        self.path
            .read_dir()
            .unwrap()
            .map(|path| path.unwrap().path())
            .collect()
    }

    pub fn search(&self, term: String) -> Option<usize> {
        self
            .children()
            .iter()
            .position(|name| name.name().to_lowercase().starts_with(&term.to_lowercase()))
    }
}
#[derive(Debug)]
struct TextFile {
    path: PathBuf,
}

pub trait FolderPath {
    fn file_type(&self) -> Type;

    fn name(&self) -> String;
}

impl FolderPath for PathBuf {


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


}