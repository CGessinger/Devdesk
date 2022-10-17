use std::{sync::Mutex, collections::HashMap};

use rusqlite::{Connection};

use crate::stats_api;

pub struct AppState {
    pub db: Database,
    pub languages: HashMap<String, Vec<String>>,
  }
  
  impl Default for AppState {
      fn default() -> Self {
          Self { 
            db: Database::new(),
            languages: serde_json::from_str(stats_api::LAN_JSON).unwrap(),
          }
      }
}

pub struct Database {
    pub conn: Mutex<Connection>
}

impl Database {
    pub fn new() -> Self {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("CREATE TABLE project (
                      id              INTEGER PRIMARY KEY,
                      name            TEXT,
                      description     TEXT,
                      path            TEXT,
                      type            TEXT
                      )", []).unwrap();
        Self {
            conn: Mutex::new(conn)
        }
    }
}
  