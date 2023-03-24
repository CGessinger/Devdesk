use std::fs;
use std::path::{Path, PathBuf};

use super::defaults;

const CONFIG_FOLDER: &str = ".devdesk";
pub fn config_path_from(path: &Path) -> PathBuf {
    path.join(CONFIG_FOLDER)
}

const FABRIC_FOLDER: &str = "fabric";
pub fn fabric_path_from(path: &Path) -> PathBuf {
    config_path_from(path).join(FABRIC_FOLDER)
}

const SCRIPTS_FOLDER: &str = "scripts";
pub fn scripts_path_from(path: &Path) -> PathBuf {
    let filename = path.file_name().unwrap();
    if filename == CONFIG_FOLDER {
        return path.join(SCRIPTS_FOLDER);
    }
    config_path_from(path).join(SCRIPTS_FOLDER)
}

const INNER_FOLDERS: &[&str] = &[SCRIPTS_FOLDER, FABRIC_FOLDER];

pub fn build_config_folders(vault_path: &Path) {
    let config_path = config_path_from(vault_path);
    fs::create_dir_all(&config_path).unwrap();
    for folder in INNER_FOLDERS {
        let inner_path = config_path.join(folder);
        fs::create_dir_all(&inner_path).unwrap();
    }
}

pub fn write_default_files(vault_path: &Path) {
    let config_path = config_path_from(vault_path);
    write_default_indicators(&config_path.join(FABRIC_FOLDER));
}

fn write_default_indicators(fabric_path: &Path) {
    let file_path = fabric_path.join("project-indicators");
    if !file_path.exists() {
        fs::write(file_path, defaults::PROJECT_INDICATORS.join("\n")).unwrap();
    }
}

pub fn read_project_indicators(fabric_path: &Path) -> Vec<String> {
    let file_path = fabric_path.join("project-indicators");
    let content = fs::read_to_string(file_path).unwrap();
    content.split("\n").map(|str| str.to_string()).collect()
}
