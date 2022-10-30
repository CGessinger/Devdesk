use std::{path::PathBuf, sync::Mutex, fs};
use rusqlite::{Connection, params};

use crate::files_api::guess_if_is_project;

pub struct Database {
    pub conn: Mutex<Connection>
}

impl Database {
    pub fn new() -> Self {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("CREATE TABLE entry (
                      id              INTEGER PRIMARY KEY,
                      path            TEXT,
                      isproject       INTEGER
                      )", []).unwrap();
        Self {
            conn: Mutex::new(conn)
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct Entry {
    pub path: String,
    pub isproject: bool,
}

#[tauri::command]
pub fn load_recursive(path: String, maxrecursion: usize, db: tauri::State<'_, Database>) -> Result<(), String>{
    let path_buf = PathBuf::from(path);

    fn recursive_inner_function (path: PathBuf, maxrecursion: usize, db: &Database) -> Result<(), String> {
        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let e = entry.map_err(|e| e.to_string())?;
            if e.metadata().unwrap().is_file() {
                continue;
            }
            let is_project = guess_if_is_project(e.path()).map_err(|e| e.to_string())?;
            let db_entry = Entry {
                path: e.path().to_str().unwrap().to_string(),
                isproject: is_project
            };
            insert_entry(db_entry, db);
            if maxrecursion > 0 && !is_project {
                recursive_inner_function(e.path(), maxrecursion - 1, &db)?;
            }
        }
        Ok(())
    }

    recursive_inner_function(path_buf, maxrecursion, &db)
}

pub fn insert_entry(entry: Entry, db: &Database) {
    let conn = db.conn.lock().unwrap();
    conn.execute("INSERT INTO entry (path, isproject)
                  VALUES (?1, ?2)",
                 params![entry.path, entry.isproject as i32]).unwrap();
}

#[tauri::command]
pub fn query_database(query: String, db: tauri::State<'_, Database>) -> Result<Vec<Entry>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare(&query).unwrap();
    let mut entries = Vec::new();
    let entry_iter = stmt.query_map([], |row| {
        Ok(Entry {
            path: row.get(1)?,
            isproject: row.get(2)?,
        })
    }).unwrap();
    for entry in entry_iter {
        entries.push(entry.unwrap());
    }
    Ok(entries)
}

#[tauri::command]
pub fn clear_db(db: tauri::State<'_, Database>) -> Result<usize, String> {
    let conn = db.conn.lock().unwrap();
    conn.execute("DELETE from entry", ()).map_err(|e| e.to_string())
}