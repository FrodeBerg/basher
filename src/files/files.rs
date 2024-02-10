use std::path::PathBuf;

pub enum File {
    Folder(Folder),
}
#[derive(Debug)]
pub struct Folder {
    pub children: Vec<String>,
    pub cursor: usize,
    pub name: String,
    pub parent: Option<Box<Folder>>,
}

impl Folder {
    pub fn root() -> Self {
        let root = "/".to_string();
        Folder {
            children: children(PathBuf::from(&root)),
            cursor: 0,
            name: root,
            parent: None,
        }
    }

    pub fn selected(&self) -> File {
        let name = self.children.get(self.cursor).unwrap();
        let path = self.path_name() + name;
        File::Folder(Folder {
            children: children(PathBuf::from(&(path))),
            name: name.to_string(),
            cursor: 0,
            parent: None,
        })
    }

    pub fn path_name(&self) -> String{
        match &self.parent {
            None => self.name.clone(),
            Some(p) => p.path_name() + &self.name, 
        }
    }
}

fn children(path: PathBuf) -> Vec<String> {
    let mut folder_names: Vec<String> = match path.read_dir() {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .filter_map(|file| file.file_name().into_string().ok())
            .collect(),
        Err(_) => Vec::new(), 
    };
    folder_names.sort();
    folder_names
}

impl File {
    pub fn path_name(&self) -> String {
        match self {
            File::Folder(folder) => {  
                folder.path_name()
            },
        }
    } 
}



