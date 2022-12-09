use chrono::prelude::{DateTime, Utc};
use std::{fs, path::Path};

pub fn is_file(file: &fs::DirEntry) -> bool {
    let meta = file.metadata();
    match meta {
        Ok(m) => m.is_file(),
        _ => false,
    }
}

pub fn modified_from(file: &fs::DirEntry) -> String {
    if let Ok(meta) = file.metadata() {
        if let Ok(modified) = meta.modified() {
            let dt: DateTime<Utc> = modified.into();
            return format!("{}", dt.format("%Y-%m-%d"));
        }
    }
    return String::from("Idk");
}

pub fn name_from(file_path: &Path) -> String {
    file_path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn guess_is_project(folder: &fs::DirEntry, indicators: &Vec<String>) -> bool {
    let path = folder.path();
    let dir_entries = fs::read_dir(path);
    if dir_entries.is_err() {
        return false;
    }

    for entry in dir_entries.unwrap() {
        if entry.is_err() {
            continue;
        }
        let file = entry.unwrap();
        if is_file(&file) {
            let name = name_from(&file.path());
            if indicators.contains(&name) {
                return true;
            }
        }
    }
    return false;
}

pub fn read_readme(project_path: &Path) -> String {
    let readme_path = project_path.join("README.md");
    if !readme_path.exists() {
        return "".to_string();
    }
    let readme = fs::read_to_string(readme_path);
    if readme.is_err() {
        return "".to_string();
    }
    return readme.unwrap();
}
