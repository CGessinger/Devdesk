use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use tauri::api::dialog::FileDialogBuilder;
use tauri::async_runtime::Mutex;

use crate::core::database::Db;
use crate::core::filesystem;
use crate::core::models::Node;
use crate::core::settings::Settings;

pub struct AppState {
    pub database: Mutex<Db>,
    pub settings: Mutex<Settings>,
}

#[tauri::command]
pub async fn backup_vault(
    app_state: tauri::State<'_, AppState>,
    window: tauri::Window,
) -> Result<(), ()> {
    let mut db = app_state.database.lock().await;
    let vault = db.get_by_id(1).await.unwrap();
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
    Ok(())
}

/*
   Safe API
*/

// ToDo rename
#[tauri::command]
pub async fn focus_project(
    id: i64,
    app_state: tauri::State<'_, AppState>,
    window: tauri::Window,
) -> Result<(), ()> {
    let mut db = app_state.database.lock().await;
    let node = db.get_by_id(id).await.unwrap();

    window.emit("spotlight_changed", &node).unwrap();
    Ok(())
}

/*
   Database
*/
#[tauri::command]
pub async fn get_vault(app_state: tauri::State<'_, AppState>) -> Result<Node, ()> {
    let mut db = app_state.database.lock().await;
    Ok(db.get_by_id(1).await.unwrap())
}

#[tauri::command]
pub async fn get_all(app_state: tauri::State<'_, AppState>) -> Result<Vec<Node>, ()> {
    let mut db = app_state.database.lock().await;
    Ok(db.get_all().await.unwrap())
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

#[tauri::command]
pub fn open(url: &Path) {
    open::that(url).unwrap();
}
