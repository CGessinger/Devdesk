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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Project {
    name: String,
    description: String,
    path: String,
    #[serde(rename = "type")]
    ptype: String,
}

#[tauri::command]
pub fn insert_project(db: tauri::State<'_, Database>, project: Project) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    conn.execute("INSERT INTO project (name, description, path, type)
                VALUES (?1, ?2, ?3, ?4)",
                &[&project.name, &project.description, &project.path, &project.ptype])
                .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn insert_projects(db: tauri::State<'_, Database>, projects: Vec<Project>) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    for p in projects {
        conn.execute("INSERT INTO project (name, description, path, type)
                    VALUES (?1, ?2, ?3, ?4)",
                    &[&p.name, &p.description, &p.path, &p.ptype])
                    .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn get_projects_filter(filter: String, db: tauri::State<'_, Database>) -> Result<Vec<Project>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare(&filter).map_err(|e| e.to_string())?;
    let project_iter = stmt.query_map([], |row| {
        Ok(Project {
            name: row.get(1)?,
            description: row.get(2)?,
            path: row.get(3)?,
            ptype: row.get(4)?
        })
    }).map_err(|e| e.to_string());

    let mut projects = Vec::new();
    for project in project_iter.unwrap() {
        projects.push(project.map_err(|e| e.to_string())?);
    }
    Ok(projects)
}

#[tauri::command]
pub fn get_projects(db: tauri::State<'_, Database>) -> Result<Vec<Project>, String> {
    get_projects_filter("SELECT * FROM project".to_string(), db)
}

#[tauri::command]
pub fn clear_db(db: tauri::State<'_, Database>) -> Result<usize, String> {
    let conn = db.conn.lock().unwrap();
    conn.execute("DELETE from project", ()).map_err(|e| e.to_string())
}