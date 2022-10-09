use std::sync::Mutex;

use rusqlite::{Connection, Result};

pub struct Database {
    pub conn: Mutex<Connection>
}

impl Database {
    pub fn new() -> Self {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("CREATE TABLE project (
                      id              INTEGER PRIMARY KEY,
                      name            TEXT NOT NULL,
                      description     TEXT
                      )", []).unwrap();
        Self {
            conn: Mutex::new(conn)
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Project {
    name: String,
    description: String,
}

#[tauri::command]
pub fn insert_project(db: tauri::State<'_, Database>, project: Project) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    conn.execute("INSERT INTO project (name, description)
                  VALUES (?1, ?2)",
                 &[&project.name, &project.description]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn insert_projects(db: tauri::State<'_, Database>, projects: Vec<Project>) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    for p in projects {
        conn.execute("INSERT INTO project (name, description)
                      VALUES (?1, ?2)",
                     &[&p.name, &p.description]).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn get_projects(db: tauri::State<'_, Database>) -> Result<Vec<Project>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM project").map_err(|e| e.to_string())?;
    let project_iter = stmt.query_map([], |row| {
        Ok(Project {
            name: row.get(1)?,
            description: row.get(2)?,
        })
    }).map_err(|e| e.to_string());

    let mut projects = Vec::new();
    for project in project_iter.unwrap() {
        projects.push(project.map_err(|e| e.to_string())?);
    }
    Ok(projects)
}

#[tauri::command]
pub fn clear_db(db: tauri::State<'_, Database>) -> Result<usize, String> {
    let conn = db.conn.lock().unwrap();
    conn.execute("DELETE from project", ()).map_err(|e| e.to_string())
}