#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use devdesk::core::database::Db;
use devdesk::core::types::Vault;
use devdesk::glue::{tauri_commands, InitResponse};
use std::path::Path;
use std::sync::Mutex;

struct App {
    db: Mutex<Db>,
    vault_id: Mutex<i64>,
}

#[tauri::command]
fn get_init_info(app: tauri::State<'_, App>) -> InitResponse {
    let db = app.db.lock().unwrap();
    let id = app.vault_id.lock().unwrap();
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
fn focus_vault(id: i64, app: tauri::State<'_, App>, window: tauri::Window) {
    *app.vault_id.lock().unwrap() = id;
    let info = get_init_info(app);
    window.emit("current_vault_change", &info);
}

#[tauri::command]
fn focus_project(id: Option<i64>, app: tauri::State<'_, App>, window: tauri::Window) {
    if let Some(id) = id {
        let db = app.db.lock().unwrap();
        db.update_project_timestamp(id);
    }

    let mut info = get_init_info(app);
    info.selected_id = id;
    window.emit("current_vault_change", &info);
}

#[tauri::command]
fn reset_current_vault(app: tauri::State<'_, App>, window: tauri::Window) {
    focus_vault(1, app, window);
}

fn main() {
    let path = Path::new("/Users/cgessinger/Documents/Programming");
    let vault = Vault::top_level(path);
    let db = Db::new(path);

    // db.clear_db();
    db.fill_with_vault(&vault).unwrap();
    let app = App {
        db: Mutex::new(db),
        vault_id: Mutex::new(1),
    };
    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![
            get_init_info,
            reset_current_vault,
            focus_vault,
            focus_project,
            tauri_commands::terminal_at,
            tauri_commands::editor_at,
            tauri_commands::open,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
