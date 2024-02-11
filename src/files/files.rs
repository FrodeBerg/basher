use std::path::PathBuf;
use crate::files::{folder::Folder, text_file::TextFile};
pub enum File {
    Folder(Folder),
    TextFile(TextFile),
}

impl File {
    pub fn path_name(&self) -> String {
        match self {
            File::Folder(folder) => {  
                folder.path_name()
            },
            File::TextFile(file) => {
                file.parent.as_ref().unwrap().path_name() + &file.name
            }
        }
    } 

    pub fn name(&self) -> String {
        match self {
            File::Folder(folder) => folder.name.clone(),
            File::TextFile(file) => file.name.clone(),
        }
    }
}





