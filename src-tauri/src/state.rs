use std::{path::Path, sync::Mutex};

use crate::core::{database::Db, settings::Settings, types::Vault};

pub struct AppState {
    pub settings: Mutex<Settings>,
    pub database: Mutex<Db>,
    pub vault_id: Mutex<i64>,
}
impl AppState {
    pub fn new(path: &Path, settings: Settings) -> Self {
        let vault = Vault::top_level(path);
        let db = Db::new(path);
        db.fill_with_vault(&vault).unwrap();

        let app_state = AppState {
            database: Mutex::new(db),
            settings: Mutex::new(settings),
            vault_id: Mutex::new(1),
        };

        app_state
    }
}

pub struct InitState {
    pub settings: Mutex<Settings>,
}
