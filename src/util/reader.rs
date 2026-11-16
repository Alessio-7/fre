use super::fileinfo::{FileInfo, FileType};
use color_eyre::{Result, eyre::eyre};
use std::{fs::{self, File, FileType as _FileType}, io::BufReader, path::PathBuf};
use serde_json::{Value, from_reader as read_json};

fn convert_file_type(ftype: _FileType, fname: &str) -> FileType {
    if ftype.is_dir() {
        FileType::Dir
    } else if ftype.is_symlink() {
        FileType::Symlink
    } else {
        let i = fname.rfind('.').unwrap_or(fname.len() - 1);
        if i < fname.len() - 1 {
            FileType::File(fname[i + 1..].to_string())
        } else {
            FileType::Error
        }
    }
}
pub(super) fn read_file(
    path: &String,
    sort_fun: fn(Vec<FileInfo>) -> Vec<FileInfo>,
) -> FileInfo {
    let i = path.rfind('/').unwrap_or(0);
    let file_name = path.get(i + 1..).unwrap_or(path);
    let file_type = match fs::metadata(path) {
        Ok(md) => convert_file_type(md.file_type(), file_name),
        Err(_) => FileType::Error,
    };

    

    let content = if file_type.is_dir() {
        match fs::read_dir(path) {
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
        match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => format!("Could not read \n{}", e),
        }
    };

    FileInfo {
        file_type,
        name: file_name.to_string(),
        content,
    }
}

pub(super) fn read_from_path(path: &String) -> Result<Vec<FileInfo>> {
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

pub fn default_config() -> Value {
    let default = include_str!("sample_conf.json");
    serde_json::from_str(default).unwrap()
}

pub fn read_config() -> Result<Value> {
    let default = include_str!("sample_conf.json");

    let mut p = PathBuf::from(std::env::var("HOME").unwrap());
    p.push(".config/fre");
    fs::create_dir_all(&p)?;
    p.push("conf.json");
    let Ok(f) = File::open(&p) else {
        let _ = fs::write(p, default);
        return Ok(default_config());
    };
    
    read_json(BufReader::new(f))
        .map_err(|e| eyre!(e.to_string()))
}
