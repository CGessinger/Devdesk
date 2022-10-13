use crate::state_api::AppState;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Project {
    name: String,
    description: String,
    path: String,
    #[serde(rename = "type")]
    ptype: String,
}

#[tauri::command]
pub fn insert_project(state: tauri::State<'_, AppState>, project: Project) -> Result<(), String> {
    let db = &state.db;
    let conn = db.conn.lock().unwrap();
    conn.execute("INSERT INTO project (name, description, path, type)
                VALUES (?1, ?2, ?3, ?4)",
                &[&project.name, &project.description, &project.path, &project.ptype])
                .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn insert_projects(state: tauri::State<'_, AppState>, projects: Vec<Project>) -> Result<(), String> {
    let db = &state.db;
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
pub fn get_projects_filter(state: tauri::State<'_, AppState>, filter: String) -> Result<Vec<Project>, String> {
    let db = &state.db;
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
pub fn get_projects(state: tauri::State<'_, AppState>) -> Result<Vec<Project>, String> {
    get_projects_filter(state, "SELECT * FROM project".to_string())
}

#[tauri::command]
pub fn clear_db(state: tauri::State<'_, AppState>) -> Result<usize, String> {
    let db = &state.db;
    let conn = db.conn.lock().unwrap();
    conn.execute("DELETE from project", ()).map_err(|e| e.to_string())
}