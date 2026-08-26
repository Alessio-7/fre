use super::fileinfo::{FileInfo, FileType};
use color_eyre::Result;
use std::fs::{self, FileType as _FileType};

fn convert_file_type(ftype: _FileType, fname: &str) -> FileType {
    if ftype.is_dir() {
        FileType::Dir
    } else if ftype.is_symlink() {
        FileType::Symlink
    } else {
        let i = fname.rfind('.').unwrap_or(fname.len());
        if i<fname.len()-1{
            FileType::File(fname.get(i+1..).unwrap().to_string())
        }else {
            FileType::UknownFile
        }
    }
}

#[derive(Debug, Default)]
pub struct Reader;
impl Reader {
    pub(super) fn read_file(
        &self,
        path: &String,
        sort_fun: fn(Vec<FileInfo>) -> Vec<FileInfo>,
    ) -> FileInfo {
        let i = path.rfind('/').unwrap_or(0);
        let file_name = path.get(i + 1..).unwrap_or(path);
        let file_type = match fs::metadata(path) {
            Ok(md) => convert_file_type(md.file_type(), file_name),
            Err(_) => FileType::UknownFile,
        };

        let content;

        if file_type.is_dir() {
            content = match fs::read_dir(path) {
                Ok(rd) => sort_fun(
                    rd.filter_map(|entry| {
                        let entry = entry.ok()?;
                        let file_type = entry.file_type().ok()?;
                        let file_name = entry.file_name().into_string().ok()?;

                        Some(FileInfo::new(
                            convert_file_type(file_type, &file_name),
                            file_name,
                        ))
                    })
                    .collect(),
                )
                .iter()
                .map(|info| format!("{}  {}\n", info.file_type.icon(), info.name))
                .collect(),
                Err(_) => "Could not read dir contents".to_string(),
            }
        } else {
            content = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(e) => format!("Could not read \n{}", e),
            };
        }

        FileInfo {
            file_type: file_type,
            name: file_name.to_string(),
            content: content,
        }
    }

    pub(super) fn read_from_path(&self, path: &String) -> Result<Vec<FileInfo>> {
        let entries = fs::read_dir(path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_type = entry.file_type().ok()?;
                let file_name = entry.file_name().into_string().ok()?;

                let new_file_type = convert_file_type(file_type, &file_name);
                Some(FileInfo::new(new_file_type, file_name))
            })
            .collect();

        Ok(entries)
        //TODO leggi le cose nel path e quelle nelle prime N cartelle tramite single_read_from_path
    }

    /*
    fn single_read_from_path(&self, path: &String) -> Vec<FileInfo> {
        Vec::new()
        //leggi solo le cose in questo path
    }
     */
}
