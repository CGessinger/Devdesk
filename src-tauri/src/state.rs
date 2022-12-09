use std::{path::Path, sync::Mutex};

use crate::core::{
    commands, database::Db, filesystem::configtree::scripts_path_from, settings::Settings,
    types::Vault,
};

pub struct AppState {
    pub settings: Mutex<Settings>,
    pub database: Mutex<Db>,
    pub scripts: Mutex<Vec<(String, String)>>,
    pub vault_id: Mutex<i64>,
}
impl AppState {
    pub fn new(path: &Path, settings: Settings) -> Self {
        let vault = Vault::top_level(path);
        let scripts_path = scripts_path_from(&vault.path);
        let scripts = commands::custom::read_custom_scripts(&scripts_path);
        let config_path = vault.path.join(".devdesk");
        let db = Db::new(&config_path);
        db.fill_with_vault(&vault).unwrap();

        let app_state = AppState {
            database: Mutex::new(db),
            settings: Mutex::new(settings),
            scripts: Mutex::new(scripts),
            vault_id: Mutex::new(1),
        };

        app_state
    }
}

pub struct InitState {
    pub settings: Mutex<Settings>,
}
