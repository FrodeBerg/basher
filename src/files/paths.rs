use std::path::PathBuf;

pub trait FolderPath {
    fn parent_folder(&self) -> PathBuf;

    fn child(&self, index: usize) -> PathBuf; 

    fn children(&self) -> Vec<String>; 

    fn path_name(&self) -> String; 

    fn selected_name(&self) -> String;

    fn search(&self, term: String) -> i16;
}

impl FolderPath for PathBuf {
    fn path_name(&self) -> String {
        let path = self.to_str().unwrap().to_string();
        path.clone() + if self.is_dir() && path != "/" {"/"} else {""}
    }

    fn children(&self) -> Vec<String> {
        let mut folder_names: Vec<String> = self
            .read_dir()
            .unwrap()
            .map(|path| path.unwrap().file_name().to_str().unwrap().to_string())
            .collect();
        folder_names.sort();
        folder_names
    }

    fn parent_folder(&self) -> PathBuf {
        match self.parent() {
            None => PathBuf::from("/"),
            Some(p) => p.to_path_buf(),
        }
    }

    fn child(&self, index: usize) -> PathBuf {
        let mut current = PathBuf::from(self.path_name());
        current.push(self.children().get(index).unwrap().to_string());
        current
    }

    fn selected_name(&self) -> String {
        self.file_name().unwrap().to_str().unwrap().to_string()
    }

    fn search(&self, term: String) -> i16 {
        self
            .children()
            .iter()
            .enumerate()
            .rev()
            .fold(-1, |file, (i, name)| if name.to_lowercase().starts_with(&term.to_lowercase()) {i as i16} else {file})
    }
}