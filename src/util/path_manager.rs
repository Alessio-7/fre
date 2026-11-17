use super::fileinfo::FileInfo;
use super::reader;
use color_eyre::Result;

fn sort(mut v: Vec<FileInfo>) -> Vec<FileInfo> {
    v.sort_by_key(|info| (!info.file_type.is_dir(), info.name.to_lowercase()));
    v
}

fn concat_path(path1: &String, path2: &String) -> String {
    if path1.ends_with('/') {
        format!("{}{}", path1, path2)
    } else {
        format!("{}/{}", path1, path2)
    }
}

#[derive(Debug, Default)]
pub struct PathManager {
    pub path: String,
    pub selected_index: usize,
    last_selected_index: usize,
    dir_list: Vec<FileInfo>,
    filtered_dir_list: Vec<usize>,
    filter: String,

    do_cut: bool,
    copy_path: String,
}

impl PathManager {
    pub fn clear_filter(&mut self) {
        self.filter = "".to_string();
        self.filtered_dir_list = (0..self.dir_list.len()).collect();
        self.selected_index = 0;
    }

    pub fn load_path(&mut self, path: String) -> Result<()> {
        self.dir_list = sort(reader::read_from_path(&path)?);
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
        let re = regex::Regex::new(&format!(r"(?i)^[^a-z]*{}", self.filter)).unwrap();
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
        if self.filter.is_empty() {
            None
        } else {
            Some(self.filter.clone())
        }
    }

    fn get_selected(&self) -> Option<&FileInfo> {
        let i = self.filtered_dir_list.get(self.selected_index)?;
        self.dir_list.get(*i)
    }

    fn get_selected_path(&self) -> Option<String> {
        let i = self.filtered_dir_list.get(self.selected_index)?;
        let info = self.dir_list.get(*i)?;
        Some(concat_path(&self.path, &info.name))
    }

    fn get_selected_and_path(&self) -> Option<(&FileInfo, String)> {
        let i = self.filtered_dir_list.get(self.selected_index)?;
        let info = self.dir_list.get(*i)?;
        Some((info, concat_path(&self.path, &info.name)))
    }

    pub fn get_dir_list(&self) -> Vec<&FileInfo> {
        self.filtered_dir_list
            .iter()
            .map(|i| &self.dir_list[*i])
            .collect()
    }

    pub fn get_file_preview(&self) -> FileInfo {
        //TODO loader and see if already loaded
        if let Some(r) = self.get_selected_path() {
            reader::read_file(&r, sort)
        } else {
            FileInfo::empty()
        }
    }

    pub fn select_first(&mut self) {
        self.selected_index = 0;
    }

    pub fn select_last(&mut self) {
        if !self.filtered_dir_list.is_empty() {
            self.selected_index = self.filtered_dir_list.len() - 1;
        }
    }

    pub fn select_previous(&mut self) {
        if !self.filtered_dir_list.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.filtered_dir_list.len() - 1
            } else {
                self.selected_index -= 1
            }
        }
    }

    pub fn select_next(&mut self) {
        if !self.filtered_dir_list.is_empty() {
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

        let p: &str = if self.path.ends_with('/') {
            self.path.get(..self.path.len() - 1).unwrap()
        } else {
            &self.path
        };
        let i = p.rfind('/').unwrap();
        let pnew = self.path.get(..i).unwrap().to_string();
        if pnew.is_empty() {
            self.load_path("/".to_string())?;
        } else {
            self.load_path(pnew)?;
        }
        self.selected_index = self.last_selected_index;
        Ok(())
    }

    pub fn go_into(&mut self) -> Result<()> {
        if let Some((info, p)) = self.get_selected_and_path()
            && info.file_type.is_dir()
        {
            self.last_selected_index = self.selected_index;
            self.selected_index = 0;
            self.load_path(p)?;
        }
        Ok(())
    }

    pub fn cut_selected(&mut self) -> Result<()> {
        if let Some(p) = self.get_selected_path() {
            self.do_cut = true;
            self.copy_path = p;
        }
        Ok(())
    }

    pub fn copy_selected(&mut self) -> Result<()> {
        if let Some(p) = self.get_selected_path() {
            self.do_cut = false;
            self.copy_path = p;
        }
        Ok(())
    }

    pub fn paste_selected(&mut self) -> Result<()> {
        if !self.copy_path.is_empty() {
            let file_name = self.copy_path.rsplit('/').next().unwrap();
            let new_path = concat_path(&self.path, &file_name.to_string());
            if self.do_cut {
                std::fs::rename(&self.copy_path, &new_path)?;
            } else {
                std::fs::copy(&self.copy_path, &new_path)?;
            }
            self.load_path(self.path.clone())?;
        }
        Ok(())
    }

    pub fn delete_selected(&mut self) -> Result<()> {
        if let Some((info, p)) = self.get_selected_and_path() {
            if info.file_type.is_dir() {
                std::fs::remove_dir_all(p)?;
            } else {
                std::fs::remove_file(p)?;
            }
            self.load_path(self.path.clone())?;
        }
        Ok(())
    }

    pub fn open_selected(&self) -> Result<()> {
        if let Some((info, p)) = self.get_selected_and_path() {
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
        if let Some(p) = self.get_selected_path() {
            open::with_detached(p, "nemo")?;
        }
        Ok(())
    }
}
