use std::path::{Path, PathBuf};

// Vault
#[derive(serde::Serialize, Debug)]
pub struct Vault {
    pub vault_id: i64,        // Id in databse
    pub path: PathBuf,        // Path to filesystem
    pub parent_vault_id: i64, // Id of parent Vault
}
impl Vault {
    pub fn top_level(path: &Path) -> Self {
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
    pub modified: String,
    pub vault_id: i64,
}
