use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use tauri::api::dialog::FileDialogBuilder;
use tauri::Manager;

use crate::app_windows::get_desk;
use crate::core::filesystem;
use crate::core::settings::Settings;
use crate::core::types::{Project, Vault};
use crate::state::AppState;

use super::DasboardResponse;

#[tauri::command]
pub fn open(url: &Path) {
    open::that(url).unwrap();
}

#[tauri::command]
pub fn pick_vault(app_handle: tauri::AppHandle) {
    FileDialogBuilder::new().pick_folder(move |folder_path| {
        if folder_path.is_none() {
            return;
        }
        let folder_path = folder_path.unwrap();

        let path_resolver = app_handle.path_resolver();
        let config_path = path_resolver.app_config_dir().unwrap();

        // Update Settings
        let mut settings = Settings::get_or_default(&config_path);
        settings.set_vault_path(folder_path.clone());
        // Save updated settings
        settings.save(config_path);

        // Construct App State
        let app_state = AppState::new(folder_path.as_path());
        app_handle.manage(app_state);

        app_handle.get_window("welcome").unwrap().close().unwrap();
        let window_builder = get_desk(&app_handle);
        window_builder.build().unwrap();
    });
}

#[tauri::command]
pub fn get_init_info(app_state: tauri::State<'_, AppState>) -> DasboardResponse {
    let db = app_state.database.lock().unwrap();
    let id = app_state.vault_id.lock().unwrap();
    let vault = db.select_vault(*id).unwrap();
    let projects = db.select_projects_under_vault(*id);
    let recent = db.select_recent_projects();
    let sub_directories = db.select_vaults_with_parent(*id);
    DasboardResponse {
        vault,
        sub_directories,
        projects,
        recent,
        selected: None,
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
    let mut project: Option<Project> = None;
    if let Some(id) = id {
        let db = app_state.database.lock().unwrap();
        db.update_project_timestamp(id);
        project = db.select_project(id);
    }

    let mut info = get_init_info(app_state);
    info.selected = project;
    window.emit("current_vault_change", &info).unwrap(); // deprecated
    window.emit("spotlight_changed", &info.selected).unwrap();
}

#[tauri::command]
pub fn reset_current_vault(app_state: tauri::State<'_, AppState>, window: tauri::Window) {
    focus_vault(1, app_state, window);
}

#[tauri::command]
pub fn backup_vault(app_state: tauri::State<'_, AppState>, window: tauri::Window) {
    let db = app_state.database.lock().unwrap();
    let id = app_state.vault_id.lock().unwrap();
    let vault = db.select_vault(*id).unwrap();
    let path = PathBuf::from(vault.path);

    FileDialogBuilder::new().pick_folder(move |folder_path| {
        if folder_path.is_none() {
            return;
        }
        window.emit("vault_backup_start", {}).unwrap();

        let size = filesystem::utils::calculate_vault_size(&path);
        window.emit("vault_size", &size).unwrap();

        let folder_path = folder_path.unwrap();
        let backup_path = PathBuf::from(folder_path);
        filesystem::utils::backup_vault(&path, &backup_path, |progress| {
            window.emit("vault_backup_progress", &progress).unwrap();
        });
        window.emit("vault_backup_end", {}).unwrap();
    });
}

#[tauri::command]
pub fn get_vault(app_state: tauri::State<'_, AppState>) -> Vault {
    let db = app_state.database.lock().unwrap();
    db.select_vault(1).unwrap()
}

#[tauri::command]
pub fn read_directory(path: String) -> Vec<String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        files.push(path.to_str().unwrap().to_string());
    }
    files
}

#[tauri::command]
pub fn is_file(path: String) -> bool {
    let path = Path::new(&path);
    path.is_file()
}

#[tauri::command]
pub fn file_exists(path: String) -> bool {
    Path::new(&path).exists()
}

#[tauri::command]
pub fn read_file(path: String) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
