use std::{path::Path, fs::{self, DirEntry}};
use chrono::prelude::{DateTime, Utc};

use crate::core::sql_library::Db;

pub fn recursive_read_to_database(db: &Db, path: &Path, parent_vault_id: i64, recursion: u16) -> std::io::Result<()> {
    let vault_id = db.insert_vault(path, parent_vault_id).unwrap();

    if recursion >= 2 {
        return Ok(());
    }

    let dir_entries = fs::read_dir(path)?;
    for entry in dir_entries {
        match entry {
            Ok(file) => if !is_file(&file) {
                let is_project = guess_is_project(&file);
                let file_path = file.path();
                if is_project {
                    let name = file_path.file_name().unwrap();
                    let modified = &modified_from(&file);
                    db.insert_project(name, modified, vault_id);
                } else {
                    recursive_read_to_database(db, &file_path, vault_id, recursion + 1);
                }
            },
            _ => continue,
        }
    }
    Ok(())
}

pub fn is_file(file: &DirEntry) -> bool {
    let meta = file.metadata();
    match meta {
        Ok(m) => m.is_file(),
        _ => false
    }
}

pub fn modified_from(file: &DirEntry) -> String {
    if let Ok(meta) = file.metadata() {
        if let Ok(modified) = meta.modified() {
            let dt: DateTime<Utc> = modified.into();
            return format!("{}", dt.format("%Y-%m-%d"));
        }
    }
    return String::from("");
}

pub fn guess_is_project(file: &DirEntry) -> bool {
    let path = file.path();
    false
}