use std::{path::Path, sync::Mutex};

use crate::core::commands;
use crate::core::database::Db;
use crate::core::filesystem::configtree;
use crate::core::types::Vault;

pub struct AppState {
    pub database: Mutex<Db>,
    pub scripts: Mutex<Vec<(String, String)>>,
    pub vault_id: Mutex<i64>,
}
impl AppState {
    pub fn new(path: &Path) -> Self {
        let vault = Vault::top_level(path);
        let scripts_path = configtree::scripts_path_from(&vault.path);
        let scripts = commands::custom::read_custom_scripts(&scripts_path);
        let config_path = vault.path.join(".devdesk");
        let db = Db::new(&config_path);
        db.fill_with_vault(&vault).unwrap();

        let app_state = AppState {
            database: Mutex::new(db),
            scripts: Mutex::new(scripts),
            vault_id: Mutex::new(1),
        };

        app_state
    }
}
