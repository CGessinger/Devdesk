use std::sync::Mutex;

use crate::core::{database::Db, settings::Settings};

pub struct AppState {
    pub settings: Mutex<Settings>,
    pub database: Mutex<Db>,
    pub vault_id: Mutex<i64>,
}

pub struct InitState {
    pub settings: Mutex<Settings>,
}
