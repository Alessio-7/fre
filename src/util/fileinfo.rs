#[derive(Debug)]
pub struct FileInfo {
    pub file_type: FileType,
    pub name: String,
    pub content: String,
}

impl FileInfo {
    pub fn new(file_type: FileType, name: String) -> Self {
        FileInfo {
            file_type: file_type,
            name: name,
            content: "".to_string(),
        }
    }

    pub fn empty_dir() -> Self {
        FileInfo {
            file_type: FileType::Dir,
            name: "Empty dir".to_string(),
            content: "The void looks at you...".to_string(),
        }
    }
}

impl Default for FileInfo {
    fn default() -> Self {
        FileInfo {
            file_type: FileType::UknownFile,
            name: "default file".to_string(),
            content: "Not human readable".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileType {
    Dir,
    Symlink,
    File(String),
    UknownFile
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

    pub fn is_dir(&self) -> bool {
        self == &FileType::Dir
    }
}
