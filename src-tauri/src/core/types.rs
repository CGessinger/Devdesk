use std::path::{Path, PathBuf};

// Vault
#[derive(serde::Serialize)]
pub struct Vault {
    pub vault_id: i64,  // Id in databse
    pub path: PathBuf,  // Path to filesystem
    pub parent_id: i64, // Id of parent Vault
}
impl Vault {
    pub fn top_level(path: &Path) -> Self {
        Self {
            vault_id: 1,
            path: PathBuf::from(path),
            parent_id: 0,
        }
    }
    pub fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            vault_id: row.get(0)?,
            path: PathBuf::from(row.get::<usize, String>(1)?),
            parent_id: row.get(3)?,
        })
    }
}

// Project
#[derive(serde::Serialize)]
pub struct Project {
    project_id: i64,
    name: String,
    vault_id: i64,
}
impl Project {
    pub fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            project_id: row.get(0)?,
            name: row.get::<usize, String>(1)?,
            vault_id: row.get(5)?,
        })
    }
}
