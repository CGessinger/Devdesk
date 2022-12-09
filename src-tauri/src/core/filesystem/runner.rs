use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::core::database::{
    types_db_interface::{ProjectInsertData, VaultInsertData},
    Db,
};

use super::utils;

pub fn recursive_read_to_database(
    db: &Db,
    path: &Path,
    parent_vault_id: i64,
    recursion: u16,
) -> std::io::Result<()> {
    let name = utils::name_from(&path);
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
        if entry.is_err() {
            continue;
        }

        let folder = entry.unwrap();
        if utils::is_file(&folder) {
            continue;
        }

        let is_project = utils::guess_is_project(&folder);
        let file_path = folder.path();
        if is_project {
            let project_data = ProjectInsertData {
                name: utils::name_from(file_path.as_path()),
                path: file_path,
                modified: utils::modified_from(&folder),
                vault_id,
            };
            db.upsert_project(project_data).unwrap();
        } else {
            recursive_read_to_database(db, &file_path, vault_id, recursion + 1).unwrap();
        }
    }
    Ok(())
}
