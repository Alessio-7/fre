use super::fileinfo::FileInfo;
use super::reader::Reader;
use color_eyre::Result;

fn sort(mut v: Vec<FileInfo>) -> Vec<FileInfo> {
    v.sort_by_key(|info| (!info.file_type.is_dir(), info.name.to_lowercase()));
    v
}

fn concat_path(path1: &String, path2: &String) -> String {
    if path1.ends_with('/') {
        return format!("{}{}", path1, path2);
    } else {
        return format!("{}/{}", path1, path2);
    }
}

#[derive(Debug, Default)]
pub struct PathManager {
    reader: Reader,
    pub path: String,
    pub selected_index: usize,
    index_stack: Vec<usize>,
    dir_list: Vec<FileInfo>,
}

impl PathManager {
    pub fn load_path(&mut self, path: String) -> Result<()> {
        self.dir_list = sort(self.reader.read_from_path(&path)?);
        self.selected_index = 0;
        self.path = path;
        Ok(())
        //TODO immagazina tutte le altre informazioni
    }

    fn get_selected_file(&self) -> Option<&FileInfo> {
        self.dir_list.get(self.selected_index)
    }

    pub fn get_dir_list(&self) -> &Vec<FileInfo> {
        &self.dir_list
    }

    pub fn get_file_preview(&self) -> FileInfo {
        //TODO loader and see if already loaded

        let e = FileInfo::empty_dir(); // TODO non fare sta cosa in qualche modo
        let r = self.get_selected_file().unwrap_or(&e);
        self.reader
            .read_file(&concat_path(&self.path, &r.name), sort)
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
        if self.path == "/" {
            return Ok(());
        }

        let p: &str;
        if self.path.ends_with('/') {
            p = self.path.get(..self.path.len() - 1).unwrap();
        } else {
            p = &self.path;
        }
        let i = p.rfind('/').unwrap();
        let pnew = self.path.get(..i).unwrap().to_string();
        if pnew.is_empty() {
            self.load_path("/".to_string())?;
        } else {
            self.load_path(pnew)?;
        }
        Ok(())
    }

    pub fn right(&mut self) -> Result<()> {
        match self.dir_list.get(self.selected_index){
            None => {},
            Some(info) => {
                let p = concat_path(&self.path, &info.name);
                if info.file_type.is_dir() {
                    self.load_path(p)?;
                }
            }
        };
        Ok(())
    }

    pub fn open_selected(&self) -> Result<()>{
        match self.dir_list.get(self.selected_index) {
            None => {},
            Some(info) => {
                let p = concat_path(&self.path, &info.name);
                if info.file_type.is_dir() {
                    open::with_detached(p, "kitty")?;
                }else {
                    open::that_detached(p)?;
                }
            }
        };
        Ok(())
    }
}
