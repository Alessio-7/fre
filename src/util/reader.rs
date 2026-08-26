use std::fs;
use std::fs::FileType as _FileType;
use std::io;

#[derive(Debug, PartialEq, Eq)]
pub enum FileType {
    Dir,
    Symlink,
    NotHumanReadable,
    //HumanReadable(String),
    //Binary,
}

impl Default for FileType {
    fn default() -> Self {
        FileType::Dir
    }
}

impl FileType {
    pub fn icon(&self) -> char {
        if self == &FileType::Dir {
            return '';
        }
        ' '
    }
}

fn convert_file_type(ftype: _FileType, fname: String) -> (FileType, String) {
    if ftype.is_dir() {
        return (FileType::Dir, fname);
    } else if ftype.is_symlink() {
        return (FileType::Symlink, fname);
    } else {
        return (FileType::NotHumanReadable, fname); //TODO
    }
}

#[derive(Debug, Default)]
pub struct Reader;
impl Reader {
    pub(super) fn read_file(&self, path: &String) -> (FileType, String) {
        (FileType::NotHumanReadable, String::from(path))
    }

    pub(super) fn read_from_path(&self, path: &String) -> io::Result<Vec<(FileType, String)>> {
        let entries = fs::read_dir(path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_type = entry.file_type().ok()?;
                let file_name = entry.file_name().into_string().ok()?;

                Some(convert_file_type(file_type, file_name))
            })
            .collect();

        Ok(entries)
        //TODO leggi le cose nel path e quelle nelle prime N cartelle tramite single_read_from_path
    }

    /*
    fn single_read_from_path(&self, path: &String) -> Vec<(FileType, String)> {
        Vec::new()
        //leggi solo le cose in questo path
    }
     */
}
