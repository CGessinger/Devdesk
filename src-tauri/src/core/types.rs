use std::path::{Path, PathBuf};

use super::filesystem::configtree::{self, fabric_path_from, read_project_indicators};

#[derive(serde::Serialize, Debug)]
pub struct VaultConfig {
    pub project_indicators: Vec<String>,
}
impl VaultConfig {
    fn from_path(vault_path: &Path) -> Self {
        let fabric_path = fabric_path_from(vault_path);
        let project_indicators = read_project_indicators(&fabric_path);
        Self { project_indicators }
    }
}

// Vault
#[derive(serde::Serialize, Debug)]
pub struct Vault {
    pub vault_id: i64,        // Id in databse
    pub path: PathBuf,        // Path to filesystem
    pub parent_vault_id: i64, // Id of parent Vault
    pub config: Option<VaultConfig>,
}
impl Vault {
    pub fn top_level(path: &Path) -> Self {
        configtree::build_config_folders(path);
        configtree::write_default_files(path);
        let config = VaultConfig::from_path(path);
        Self {
            vault_id: 1,
            path: PathBuf::from(path),
            parent_vault_id: 0,
            config: Some(config),
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
