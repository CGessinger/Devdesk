use std::{ffi::OsStr, path::Path};

use crate::fs_runner;

use super::types::{Project, Vault};

pub struct Db(rusqlite::Connection);
impl Db {
    pub fn new(path: &Path) -> Self {
        let connection = rusqlite::Connection::open(path.join("reference")).unwrap();
        connection.execute(V_CREATE_TABLE, []);
        connection.execute(P_CREATE_TABLE, []);
        connection.execute(V_SET_KEEP, [0]);
        connection.execute(P_SET_KEEP, [0]);
        Self(connection)
    }

    fn execute<P: rusqlite::Params>(&self, sql: &str, params: P) -> Result<(), String> {
        match self.0.execute(sql, params) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn fill_with_vault(&self, vault: &Vault) -> Result<(), String> {
        fs_runner::recursive_read_to_database(self, vault.path.as_path(), vault.parent_id, 0)
            .map_err(|e| e.to_string())?;
        self.execute(P_PURGE_KEEP, [])?;
        self.execute(V_PURGE_KEEP, [])?;
        Ok(())
    }

    pub fn insert_vault<S: AsRef<OsStr> + ?Sized>(
        &self,
        path: &S,
        parent_vault_id: i64,
    ) -> Result<i64, String> {
        let mut stmt = self.0.prepare_cached(V_UPSERT_OR_KEEP).unwrap();
        let res = stmt.insert([
            path.as_ref().to_str().unwrap(),
            &parent_vault_id.to_string(),
        ]);
        match res {
            Ok(i) => Ok(i),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn select_vault(&self, vault_id: i64) -> Option<Vault> {
        let res = self
            .0
            .query_row(V_SELECT_ALL_BY_ID, [vault_id], |row| Vault::from_row(row));
        match res {
            Ok(vault) => Some(vault),
            _ => None,
        }
    }

    pub fn insert_project<S: AsRef<OsStr> + ?Sized>(
        &self,
        name: &S,
        modified: &str,
        vault_id: i64,
    ) -> Result<i64, String> {
        let mut stmt = self.0.prepare_cached(P_UPSERT_OR_KEEP).unwrap();
        let res = stmt.insert([
            name.as_ref().to_str().unwrap(),
            modified,
            &vault_id.to_string(),
        ]);
        match res {
            Ok(i) => Ok(i),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn select_project(&self, project_id: i64) -> Option<Project> {
        let res = self.0.query_row(P_SELECT_ALL_BY_ID, [project_id], |row| {
            Project::from_row(row)
        });
        match res {
            Ok(project) => Some(project),
            _ => None,
        }
    }

    pub fn update_project_timestamp(&self, project_id: i64) {
        self.execute(P_UPDATE_TIMESTAMP, [project_id]).unwrap();
    }
}

const V_CREATE_TABLE: &str = "CREATE TABLE if not exists Vaults (
    VaultId INTEGER primary key,
    Path TEXT NOT NULL,
    Keep INTEGER DEFAULT 1 NOT NULL,
    ParentVaultId INTEGER NOT NULL,
    CONSTRAINT valid UNIQUE(Path)
)";
const V_SET_KEEP: &str = "UPDATE Vaults SET Keep = (?1)";
const V_PURGE_KEEP: &str = "DELETE FROM Vaults WHERE Keep = 0";
const V_SELECT_ALL_BY_ID: &str = "SELECT * FROM Vaults WHERE VaultId = (?)";
const V_SELECT_ALL_BY_PATH: &str = "SELECT VaultId from Vaults WHERE Path=(?)";
const V_UPSERT_OR_KEEP: &str = "INSERT OR IGNORE INTO Vaults (Path, ParentVaultId) values (?1, ?2)
    ON CONFLICT DO UPDATE set Keep = 1";

const P_CREATE_TABLE: &str = "CREATE TABLE if not exists Projects (
    ProjectId INTEGER primary key,
    Name TEXT not null,
    Modified TEXT not null,
    Timestamp STRING DEFAULT CURRENT_TIMESTAMP,
    Keep INTEGER DEFAULT 1 NOT NULL,
    VaultId INTEGER not null,
    FOREIGN KEY(VaultId) REFERENCES Vaults(VaultId),
    CONSTRAINT vaild UNIQUE (Name, VaultId)
)";
const P_SET_KEEP: &str = "UPDATE Projects SET Keep = (?1)";
const P_PURGE_KEEP: &str = "DELETE FROM Projects WHERE Keep = 0";
const P_UPDATE_TIMESTAMP: &str =
    "UPDATE Projects set Timestamp = CURRENT_TIMESTAMP WHERE ProjectId = (?1)";
const P_SELECT_ALL_BY_ID: &str = "SELECT * FROM Projects WHERE ProjectId = (?1)";
const P_UPSERT_OR_KEEP: &str = "INSERT INTO Projects (Name, Modified, VaultId) values (?1, ?2, ?3)
    ON CONFLICT DO UPDATE set Keep = 1, Modified = (?2)";

#[cfg(test)]
mod test {
    use std::{fs, path::Path};

    use crate::core::{sql_library::Db, types::Vault};

    fn ensure_empty_db<'a>() -> (Db, &'a Path) {
        let path = Path::new("/Users/cgessinger/Documents/Programming/maintained/devdesk/dev-test");
        fs::remove_file(path.join("reference"));
        (Db::new(path), path)
    }

    #[test]
    fn db_creation() {
        let (_, path) = ensure_empty_db();
        assert!(path.join("reference").exists())
    }

    #[test]
    fn db_fill() {
        let (db, path) = ensure_empty_db();
        let vault = Vault::top_level(path);
        let fill_success = db.fill_with_vault(&vault);
        assert!(fill_success.is_ok())
    }
}
