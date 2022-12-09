mod sql_library;
pub mod types_db_interface;
use std::path::Path;

use crate::core::filesystem;

use self::types_db_interface::{FromRow, Insertable, ProjectInsertData, VaultInsertData};

use super::{
    filesystem::utils,
    types::{Project, Vault},
};

pub struct Db(rusqlite::Connection);
impl Db {
    pub fn new(path: &Path) -> Self {
        let connection = rusqlite::Connection::open(path.join("reference")).unwrap();
        connection.execute(sql_library::V_CREATE_TABLE, []).unwrap();
        connection.execute(sql_library::P_CREATE_TABLE, []).unwrap();
        connection.execute(sql_library::V_SET_KEEP, [0]).unwrap();
        connection.execute(sql_library::P_SET_KEEP, [0]).unwrap();
        Self(connection)
    }

    fn execute<P: rusqlite::Params>(&self, sql: &str, params: P) -> Result<(), String> {
        match self.0.execute(sql, params) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn query_many<T: FromRow, P: rusqlite::Params>(&self, sql: &str, params: P) -> Vec<T> {
        let mut stmt = self.0.prepare_cached(sql).unwrap();
        let query = stmt.query(params);

        let mut t_vec: Vec<T> = Vec::new();
        // ugly
        if let Ok(mut rows) = query {
            // ugly too
            while let Ok(Some(row)) = rows.next() {
                let t = T::from_row(row);
                if t.is_ok() {
                    t_vec.push(t.unwrap());
                }
            }
        }
        t_vec
    }

    pub fn query_single<T: FromRow, P: rusqlite::Params>(&self, sql: &str, params: P) -> Option<T> {
        let res = self.0.query_row(sql, params, |row| T::from_row(row));
        match res {
            Ok(t) => Some(t),
            _ => None,
        }
    }

    fn upsert_single<I: Insertable>(&self, data: I) -> Result<i64, String> {
        data.insert(self).map_err(|e| e.to_string())
    }

    pub fn fill_with_vault(&self, vault: &Vault) -> Result<(), String> {
        let indicators = &vault.config.as_ref().unwrap().project_indicators;
        filesystem::runner::recursive_read_to_database(
            self,
            vault.path.as_path(),
            vault.parent_vault_id,
            0,
            indicators,
        )
        .map_err(|e| e.to_string())?;
        self.execute(sql_library::P_PURGE_KEEP, [])?;
        self.execute(sql_library::V_PURGE_KEEP, [])?;
        Ok(())
    }

    // Helper Function. Might delete
    pub fn upsert_vault(&self, data: VaultInsertData) -> Result<i64, String> {
        self.upsert_single(data)
    }

    pub fn select_vaults_with_parent(&self, parent_vault_id: i64) -> Vec<Vault> {
        self.query_many(sql_library::V_SELECT_ALL_BY_PARENT_ID, [parent_vault_id])
    }

    pub fn select_vault(&self, vault_id: i64) -> Option<Vault> {
        self.query_single(sql_library::V_SELECT_ALL_BY_ID, [vault_id])
    }

    fn recursive_collect_subvault_ids(&self, vault_id: i64, ids: &mut Vec<i64>) {
        ids.push(vault_id);
        let vaults = self.select_vaults_with_parent(vault_id);
        for vault in vaults {
            self.recursive_collect_subvault_ids(vault.vault_id, ids);
        }
    }

    // Helper Function. Might delete
    pub fn upsert_project(&self, data: ProjectInsertData) -> Result<i64, String> {
        self.upsert_single(data)
    }

    pub fn select_project(&self, project_id: i64) -> Option<Project> {
        self.query_single(sql_library::P_SELECT_ALL_BY_ID, [project_id])
    }

    pub fn select_recent_projects(&self) -> Vec<Project> {
        self.query_many(sql_library::P_SELECT_ALL_RECENT, [5])
    }

    pub fn select_projects_under_vault(&self, vault_id: i64) -> Vec<Project> {
        let mut ids = Vec::<i64>::new();
        self.recursive_collect_subvault_ids(vault_id, &mut ids);

        let id_query_string = ids
            .iter()
            .map(|id| format!("VaultId = {}", id))
            .collect::<Vec<String>>()
            .join(" OR ");

        if id_query_string.is_empty() {
            return Vec::new();
        }

        let sql = format!(
            "SELECT * FROM Projects WHERE {} ORDER BY VaultId asc",
            id_query_string
        );
        self.query_many(&sql, [])
    }

    pub fn update_project_timestamp(&self, project_id: i64) {
        self.execute(sql_library::P_UPDATE_TIMESTAMP, [project_id])
            .unwrap();
    }
}

mod tests;
