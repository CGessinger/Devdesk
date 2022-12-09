use std::{
    fs,
    path::{Path, PathBuf},
};

const CONFIG_FOLDER: &str = ".devdesk";
pub fn config_path_from(path: &Path) -> PathBuf {
    path.join(CONFIG_FOLDER)
}

const SCRIPTS_FOLDER: &str = "scripts";
pub fn scripts_path_from(path: &Path) -> PathBuf {
    let filename = path.file_name().unwrap();
    if filename == CONFIG_FOLDER {
        return path.join(SCRIPTS_FOLDER);
    }
    config_path_from(path).join(SCRIPTS_FOLDER)
}

const INNER_FOLDERS: &[&str] = &[SCRIPTS_FOLDER];

pub fn build_config_folders(vault_path: &Path) {
    let config_path = vault_path.join(CONFIG_FOLDER);
    fs::create_dir_all(&config_path).unwrap();
    for folder in INNER_FOLDERS {
        let inner_path = config_path.join(folder);
        fs::create_dir_all(&inner_path).unwrap();
    }
}
