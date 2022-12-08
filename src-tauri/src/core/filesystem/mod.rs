use chrono::prelude::{DateTime, Utc};
use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use crate::core::database::{
    types_db_interface::{ProjectInsertData, VaultInsertData},
    Db,
};

pub fn recursive_read_to_database(
    db: &Db,
    path: &Path,
    parent_vault_id: i64,
    recursion: u16,
) -> std::io::Result<()> {
    let name = name_from(&path);
    // ToDo add ignore file
    if name.starts_with(".") {
        return Ok(());
    }

    let vault_data = VaultInsertData {
        path: PathBuf::from(path),
        parent_vault_id,
    };
    let vault_id = db.upsert_vault(vault_data).unwrap();

    if recursion >= 3 {
        return Ok(());
    }

    let dir_entries = fs::read_dir(path)?;
    for entry in dir_entries {
        match entry {
            Ok(file) => {
                if !is_file(&file) {
                    let is_project = guess_is_project(&file);
                    let file_path = file.path();
                    if is_project {
                        let project_data = ProjectInsertData {
                            name: name_from(file_path.as_path()),
                            path: file_path,
                            modified: modified_from(&file),
                            vault_id,
                        };
                        db.upsert_project(project_data).unwrap();
                    } else {
                        recursive_read_to_database(db, &file_path, vault_id, recursion + 1)
                            .unwrap();
                    }
                }
            }
            _ => continue,
        }
    }
    Ok(())
}

pub fn is_file(file: &DirEntry) -> bool {
    let meta = file.metadata();
    match meta {
        Ok(m) => m.is_file(),
        _ => false,
    }
}

pub fn modified_from(file: &DirEntry) -> String {
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

const PROJECT_INDICATORS: &[&str] = &["src", ".prj", "lib", "bin", "Cargo.toml", "package.json"];
pub fn guess_is_project(file: &DirEntry) -> bool {
    let path = file.path();
    let dir_entries = fs::read_dir(path);
    if dir_entries.is_err() {
        return false;
    }

    for entry in dir_entries.unwrap() {
        match entry {
            Ok(file) => {
                if is_file(&file) {
                    let name = file.file_name();
                    if PROJECT_INDICATORS.contains(&name.to_str().unwrap()) {
                        return true;
                    }
                }
            }
            _ => return false,
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

const CONFIG_FOLDER: &str = ".devdesk";
const SCRIPTS_FOLDER: &str = "scripts";
pub fn config_path_from(path: &Path) -> PathBuf {
    path.join(CONFIG_FOLDER)
}

pub fn scripts_path_from(path: &Path) -> PathBuf {
    let filename = path.file_name().unwrap();
    if filename == CONFIG_FOLDER {
        return path.join(SCRIPTS_FOLDER);
    }
    config_path_from(path).join(SCRIPTS_FOLDER)
}
