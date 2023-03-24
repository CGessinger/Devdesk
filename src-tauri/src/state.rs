use std::{path::Path, sync::Mutex};

use crate::core::database::Db;
use crate::core::types::Vault;

pub struct AppState {
    pub database: Mutex<Db>,
    pub vault_id: Mutex<i64>,
}
impl AppState {
    pub fn new(path: &Path) -> Self {
        let vault = Vault::top_level(path);
        let config_path = vault.path.join(".devdesk");
        let db = Db::new(&config_path);
        db.fill_with_vault(&vault).unwrap();

        let app_state = AppState {
            database: Mutex::new(db),
            vault_id: Mutex::new(1),
        };

        app_state
    }
}
