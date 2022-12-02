use std::path::PathBuf;

use crate::core::types::{Project, Vault};

use super::{sql_library, Db};

pub trait FromRow: Sized {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self>;
}

pub trait Insertable {
    fn insert(&self, db: &Db) -> rusqlite::Result<i64>;
    fn select_row_id(&self, db: &Db) -> Option<i64>;
}

/*
   Vault
*/
impl FromRow for Vault {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            vault_id: row.get(0)?,
            path: PathBuf::from(row.get::<usize, String>(1)?),
            parent_vault_id: row.get(3)?,
        })
    }
}

pub struct VaultInsertData {
    pub path: PathBuf,
    pub parent_vault_id: i64,
}
impl Insertable for VaultInsertData {
    fn insert(&self, db: &Db) -> rusqlite::Result<i64> {
        let mut stmt = db.0.prepare_cached(sql_library::V_UPSERT_OR_KEEP)?;
        stmt.execute([
            self.path.to_str().unwrap(),
            &self.parent_vault_id.to_string(),
        ])?;
        let id = self.select_row_id(db).unwrap();
        Ok(id)
    }

    fn select_row_id(&self, db: &Db) -> Option<i64> {
        let res: Option<Vault> = db.query_single(
            sql_library::V_SELECT_ALL_BY_PATH,
            [self.path.to_str().unwrap()],
        );
        match res {
            Some(vault) => Some(vault.vault_id),
            None => None,
        }
    }
}

/*
   Project
*/
impl FromRow for Project {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            project_id: row.get(0)?,
            name: row.get::<usize, String>(1)?,
            modified: row.get::<usize, String>(3)?,
            vault_id: row.get(6)?,
        })
    }
}
pub struct ProjectInsertData {
    pub name: String,
    pub path: PathBuf,
    pub modified: String,
    pub vault_id: i64,
}
impl Insertable for ProjectInsertData {
    fn insert(&self, db: &Db) -> rusqlite::Result<i64> {
        let mut stmt = db.0.prepare_cached(sql_library::P_UPSERT_OR_KEEP)?;
        stmt.execute([
            &self.name,
            self.path.to_str().unwrap(),
            &self.modified,
            &self.vault_id.to_string(),
        ])?;
        let id = self.select_row_id(db).unwrap();
        Ok(id)
    }

    fn select_row_id(&self, db: &Db) -> Option<i64> {
        let res: Option<Project> = db.query_single(
            sql_library::P_SELECT_ALL_BY_PATH,
            [self.path.to_str().unwrap()],
        );
        match res {
            Some(project) => Some(project.project_id),
            None => None,
        }
    }
}
