use super::reader::{FileType, Reader};
use color_eyre::Result;

fn sort(mut v: Vec<(FileType, String)>) -> Vec<(FileType, String)>{
   v.sort_by_key(|(file_type, name)| (file_type != &FileType::Dir, name.to_lowercase()));
   v
}

#[derive(Debug, Default)]
pub struct PathManager {
    reader: Reader,
    pub path: String,
    pub selected_index: usize,
    dir_list: Vec<(FileType, String)>,
}

impl PathManager {

    pub fn load_path(&mut self, path: String) -> Result<()> {
        self.dir_list = sort(self.reader.read_from_path(&path)?);
        self.path = path;
        Ok(())
        //TODO immagazina tutte le altre informazioni
    }

    fn get_selected_file(&self) -> &(FileType, String) {
        self.dir_list
            .get(self.selected_index)
            .expect("è successo qualcosa che non doveva succedere")
    }

    pub fn get_dir_list(&self) -> &Vec<(FileType, String)> {
        &self.dir_list
    }

    pub fn get_file_preview(&self) -> (FileType, String) {
        self.reader.read_file(&self.get_selected_file().1)
    }

    pub fn up(&mut self) {
        if self.selected_index == 0 {
            self.selected_index = self.dir_list.len() - 1
        } else {
            self.selected_index -= 1
        }
    }

    pub fn down(&mut self) {
        self.selected_index += 1;
        if self.selected_index >= self.dir_list.len() {
            self.selected_index = 0;
        }
    }

    pub fn left(&mut self) -> Result<()> {
        if self.path == "/"{
            return Ok(())
        }

        let p: &str;
        if self.path.ends_with('/') {
            p = self.path.get(..self.path.len()-1).unwrap();
        } else {
            p = &self.path;
        }
        let i = p.rfind('/').unwrap();
        let pnew = self.path.get(..i).unwrap().to_string();
        if pnew.is_empty(){
            self.load_path("/".to_string())?;
        }else {
            self.load_path(pnew)?;    
        }
        Ok(())
    }

    pub fn right(&mut self) -> Result<()> {
        let t = &self.dir_list.get(self.selected_index);
        if t.is_some_and(|t| t.0 == FileType::Dir) {
            let p = format!("{}/{}", self.path, &t.unwrap().1);
            self.load_path(p)?;
        }
        Ok(())
    }
}
