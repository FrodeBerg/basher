use std::path::PathBuf;
use crate::files::{files::File, folder::Folder};

#[derive(Debug)]
pub struct TextFile {
    pub name: String,
    pub parent: Option<Box<Folder>>,
}