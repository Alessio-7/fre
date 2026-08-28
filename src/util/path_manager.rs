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
    dir_list: Vec<FileInfo>,
    filtered_dir_list: Vec<usize>,
    filter_letter: char,
}

impl PathManager {
    pub fn clear_filter(&mut self) {
        self.filter_letter = ' ';
        self.filtered_dir_list = (0..self.dir_list.len()).collect();
        self.selected_index = 0;
    }

    pub fn load_path(&mut self, path: String) -> Result<()> {
        self.dir_list = sort(self.reader.read_from_path(&path)?);
        self.clear_filter();
        self.path = path;
        Ok(())
        //TODO immagazina tutte le altre informazioni
    }

    pub fn filter_dir_list(&mut self, letter: char) {
        if self.filter_letter == letter {
            self.select_next();
        } else {
            self.filter_letter = letter;
            let re = regex::Regex::new(&format!(r"(?i)^[^a-z]*{}", &letter)).unwrap();
            self.filtered_dir_list.clear();
            for i in 0..self.dir_list.len() {
                let name = &self.dir_list[i].name;
                if re.find(name).is_some() {
                    self.filtered_dir_list.push(i);
                }
            }
            self.selected_index = 0;
        }
    }

    pub fn get_filter_letter(&self) -> Option<char> {
        if self.filter_letter == ' ' {
            None
        } else {
            Some(self.filter_letter)
        }
    }

    fn get_selected(&self) -> Option<&FileInfo> {
        let i = self.filtered_dir_list.get(self.selected_index)?;
        self.dir_list.get(*i)
    }

    pub fn get_dir_list(&self) -> Vec<&FileInfo> {
        self.filtered_dir_list
            .iter()
            .map(|i| &self.dir_list[*i])
            .collect()
    }

    pub fn get_file_preview(&self) -> FileInfo {
        //TODO loader and see if already loaded
        if let Some(r) = self.get_selected(){
            self.reader.read_file(&concat_path(&self.path, &r.name), sort)
        } else {
            FileInfo::empty()
        }
       
    }

    pub fn select_first(&mut self){
        self.selected_index = 0;
    }

    pub fn select_last(&mut self){
        if !self.filtered_dir_list.is_empty(){
            self.selected_index = self.filtered_dir_list.len()-1;
        }
    }

    pub fn select_previous(&mut self) {
        if !self.filtered_dir_list.is_empty(){
            if self.selected_index == 0 {
                self.selected_index = self.filtered_dir_list.len() - 1
            } else {
                self.selected_index -= 1
            }
        }
    }

    pub fn select_next(&mut self) {
        if !self.filtered_dir_list.is_empty(){
            self.selected_index += 1;
            if self.selected_index >= self.filtered_dir_list.len() {
                self.selected_index = 0;
            }
        }
    }

    pub fn go_out(&mut self) -> Result<()> {
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

    pub fn go_into(&mut self) -> Result<()> {
        match self.get_selected() {
            None => {}
            Some(info) => {
                let p = concat_path(&self.path, &info.name);
                if info.file_type.is_dir() {
                    self.load_path(p)?;
                }
            }
        };
        Ok(())
    }

    pub fn open_selected(&self) -> Result<()> {
        match self.get_selected() {
            None => {}
            Some(info) => {
                let p = concat_path(&self.path, &info.name);
                if info.file_type.is_dir() {
                    open::with_detached(p, "kitty")?;
                } else {
                    open::that_detached(p)?;
                }
            }
        };
        Ok(())
    }
}
