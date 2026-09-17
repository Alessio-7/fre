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
    filter: String,
}

impl PathManager {
    pub fn clear_filter(&mut self) {
        self.filter = "".to_string();
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

    /* 
    pub fn filter_dir_list_single_char(&mut self, letter: char) {
        if self.filter.starts_with(letter) {
            self.select_next();
        } else {
            self.filter = letter.to_string();
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
    */

    pub fn filter_dir_list_word(&mut self, letter: char) {
        self.filter.push(letter);
        let re = regex::Regex::new(&format!(r"(?i)^[^a-z]*{}", &self.filter)).unwrap();
        self.filtered_dir_list.clear();
        for i in 0..self.dir_list.len() {
            let name = &self.dir_list[i].name;
            if re.find(name).is_some() {
                self.filtered_dir_list.push(i);
            }
        }
        self.selected_index = 0;
    }

    pub fn get_filter(&self) -> Option<String> {
        if self.filter == "" {
            None
        } else {
            Some(self.filter.clone())
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
        if let Some(info) = self.get_selected() {
            let p = concat_path(&self.path, &info.name);
            if info.file_type.is_dir() {
                self.load_path(p)?;
            }
        }
        Ok(())
    }

    
    pub fn open_selected(&self) -> Result<()> {
        if let Some(info) = self.get_selected() {
            let p = concat_path(&self.path, &info.name);
            if info.file_type.is_dir() {
                open::with_detached(p, "kitty")?;
            } else {
                open::that_detached(p)?;
            }
        }
        Ok(())
    }

    pub fn go_into_or_open_selected(&mut self) -> Result<()> {
        if let Some(info) = self.get_selected() {
            if info.file_type.is_dir() {
                self.go_into()?;
            } else {
                self.open_selected()?;
            }
        }
        Ok(())
    }

    pub fn open_selected_on_explorer(&self) -> Result<()> {
        if let Some(info) = self.get_selected() {
            let p = if info.file_type.is_dir() { concat_path(&self.path, &info.name) } 
                            else { self.path.to_string() };
            open::with_detached(p, "nemo")?;
        }
        Ok(())
    }

}
