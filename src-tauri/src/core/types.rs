use std::{
    fs,
    path::{Path, PathBuf},
};

use super::{
    commands,
    filesystem::{config_path_from, scripts_path_from},
};

// Vault
#[derive(serde::Serialize, Debug)]
pub struct Vault {
    pub vault_id: i64,        // Id in databse
    pub path: PathBuf,        // Path to filesystem
    pub parent_vault_id: i64, // Id of parent Vault
}
impl Vault {
    pub fn top_level(path: &Path) -> Self {
        let config_path = config_path_from(path);
        fs::create_dir_all(&config_path).unwrap();
        let scripts_path = scripts_path_from(config_path.as_path());
        fs::create_dir_all(&scripts_path).unwrap();
        commands::prebuild::write_default_scripts(&scripts_path);
        Self {
            vault_id: 1,
            path: PathBuf::from(path),
            parent_vault_id: 0,
        }
    }
}

// Project
#[derive(PartialEq, Debug, serde::Serialize)]
pub struct Project {
    pub project_id: i64,
    pub name: String,
    pub path: String,
    pub modified: String,
    pub vault_id: i64,
}
