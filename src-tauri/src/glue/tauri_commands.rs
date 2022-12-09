use std::path::{Path, PathBuf};

use tauri::Manager;

use crate::{
    core::{commands, filesystem},
    state::{AppState, InitState},
};

use super::{InitResponse, ViewResponse};

#[tauri::command]
pub fn execute_script_by_name(
    script_name: String,
    project_id: i64,
    app_state: tauri::State<'_, AppState>,
) {
    let db = app_state.database.lock().unwrap();
    let project = db.select_project(project_id);
    if project.is_none() {
        return;
    }

    let scripts = app_state.scripts.lock().unwrap();
    let script = scripts
        .iter()
        .find(|s| s.0.eq_ignore_ascii_case(&script_name));
    if script.is_none() {
        return;
    }
    // 3. sanbox script?
    let project_path = PathBuf::from(&project.unwrap().path);
    commands::custom::execute_custom_script(&script.unwrap().1, &project_path).unwrap();
}

#[tauri::command]
pub fn open(url: &Path) {
    open::that(url).unwrap();
}

#[tauri::command]
pub fn get_project_view(project_id: i64, app_state: tauri::State<'_, AppState>) -> ViewResponse {
    let db = app_state.database.lock().unwrap();
    let project = db.select_project(project_id).unwrap();
    let project_path = PathBuf::from(project.path);
    let readme = filesystem::utils::read_readme(&project_path);
    let scripts = app_state.scripts.lock().unwrap();
    ViewResponse {
        name: project.name,
        scripts: scripts.to_vec(),
        readme,
    }
}

#[tauri::command]
pub fn get_init_info(app_state: tauri::State<'_, AppState>) -> InitResponse {
    let db = app_state.database.lock().unwrap();
    let id = app_state.vault_id.lock().unwrap();
    let vault = db.select_vault(*id).unwrap();
    let projects = db.select_projects_under_vault(*id);
    let recent = db.select_recent_projects();
    let sub_directories = db.select_vaults_with_parent(*id);
    InitResponse {
        vault,
        sub_directories,
        projects,
        recent,
        selected_id: None,
    }
}

#[tauri::command]
pub fn focus_vault(id: i64, app_state: tauri::State<'_, AppState>, window: tauri::Window) {
    *app_state.vault_id.lock().unwrap() = id;
    let info = get_init_info(app_state);
    window.emit("current_vault_change", &info).unwrap();
}

#[tauri::command]
pub fn focus_project(
    id: Option<i64>,
    app_state: tauri::State<'_, AppState>,
    window: tauri::Window,
) {
    if let Some(id) = id {
        let db = app_state.database.lock().unwrap();
        db.update_project_timestamp(id);
    }

    let mut info = get_init_info(app_state);
    info.selected_id = id;
    window.emit("current_vault_change", &info).unwrap();
}

#[tauri::command]
pub fn reset_current_vault(app_state: tauri::State<'_, AppState>, window: tauri::Window) {
    focus_vault(1, app_state, window);
}

// When this function is called, the Settings are already initilized and only need to be updated
#[tauri::command]
pub fn set_vault_path(
    path: PathBuf,
    init_state: tauri::State<'_, InitState>,
    app_handle: tauri::AppHandle,
    window: tauri::Window,
) {
    // Update Settings
    let mut settings = init_state.settings.lock().unwrap().clone();
    settings.set_vault_path(path.clone());
    let path_resolver = app_handle.path_resolver();
    let config_path = path_resolver.app_config_dir().unwrap();
    settings.save(config_path);

    // Construct App State
    let app_state = AppState::new(path.as_path(), settings);
    app_handle.manage(app_state);

    // Notify Client
    window.emit("vault_set_success", {}).unwrap();
}
