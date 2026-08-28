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

    pub fn empty() -> Self {
        FileInfo {
            file_type: FileType::Empty,
            name: "Empty".to_string(),
            content: "The void looks at you...".to_string(),
        }
    }
}

impl Default for FileInfo {
    fn default() -> Self {
        FileInfo {
            file_type: FileType::File("".to_string()),
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
    Error,
    Empty
}

impl Default for FileType {
    fn default() -> Self {
        FileType::Dir
    }
}

impl FileType {
    pub fn icon(&self) -> char {
        match &self {
            &FileType::Dir => '',
            &FileType::Symlink => '',
            &FileType::Error => '󰡯',
            &FileType::File(extension) => match extension.as_str() {
                "png" | "jpg" | "webp" | "tif" | "svg" => '󰈟',
                "txt" => '',
                "zip" | "tar" => '󰛫',
                "pdf" => '',
                "conf" | "config" | "env" | "rasi" | "toml" | "ini" => '',
                "lock" => '',
                "old" => '󰩹',
                "bin" => '',

                "git" | "gitignore" => '',
                "csv" => '',
                "xml" => '󰗀',
                "json" | "jsonc" | "yaml" => '',
                "html" | "htm" => '',
                "css" => '',
                "py" => '',
                "rs" => '',
                "js" => '',
                "java" => '',
                "rb" => '',
                "nb" => '',
                "sh" => '',
                "lua" => '',
                "c" => '',
                "cpp" => '',
                "cs" => '',
                "h" | "hpp" => '󰌷',
                "php" => '',
                "db" => '',
                "glsl" => '',
                "tex" => '',
                "sql" => '',
                "deb" => '',

                _ => ' ',
            },
            _ => ' '
        }
    }

    pub fn is_dir(&self) -> bool {
        self == &FileType::Dir
    }
}
