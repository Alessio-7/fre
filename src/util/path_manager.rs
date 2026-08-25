use super::reader::Reader;
use std::fs::FileType;

#[derive(Debug, Default)]
pub struct PathManager {
    reader: Reader,
    path: String,
    selected_index: usize,
    dir_list: Vec<(FileType, String)>,
}

impl PathManager {
    pub fn load_path(&mut self, path: String) {
        self.path = path;
        self.dir_list = self.reader.read_from_path(&self.path);
        //TODO call reader
    }

    fn get_selected_path(&self) -> &(FileType, String){
        self.dir_list
            .get(self.selected_index)
            .expect("è successo qualcosa che non doveva succedere")
    }

    pub fn get_dir_list(&self) -> *const Vec<(FileType, String)> {
        todo!("get dir list")
    }

    pub fn get_file_preview(&self) -> (FileType, String) {
        self.reader.read_file(&self.get_selected_path().1)
    }

    pub fn up(&mut self) {
        self.selected_index = (self.selected_index - 1).clamp(0, self.dir_list.len());
    }

    pub fn down(&mut self) {
        self.selected_index = (self.selected_index + 1).clamp(0, self.dir_list.len());
    }

    pub fn left(&mut self) {
        let t = &self.dir_list.get(self.selected_index);
        if t.is_some_and(|t| t.0.is_dir()) {
            let p = format!("{}/{}", self.path, &t.unwrap().1);
            self.load_path(p);
        }
    }

    pub fn right(&mut self) {}
}
