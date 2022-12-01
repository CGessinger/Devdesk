use std::path::{PathBuf, Path};


// Vault
pub struct Vault {
    pub vault_id: i64, // Id in databse
    pub path: PathBuf, // Path to filesystem
    pub parent_id: i64, // Id of parent Vault
}
impl Vault {
    pub fn top_level(path: &Path) -> Self {
        Self {
            vault_id: 1,
            path: PathBuf::from(path),
            parent_id: 0
        }
    }
    pub fn from_row(row: rusqlite::Row) -> Self {
        todo!()
    }
}

// Project
pub struct Project {
    project_id: i64,
    name: String,
    vault_id: i64,
}