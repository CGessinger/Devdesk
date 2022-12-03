#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use devdesk::app_windows::{get_desk, get_welcome};
use devdesk::core::database::Db;
use devdesk::core::settings::Settings;
use devdesk::core::types::Vault;
use devdesk::glue::{tauri_commands, InitResponse};
use devdesk::state::{AppState, InitState};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::Manager;

#[tauri::command]
fn get_init_info(app_state: tauri::State<'_, AppState>) -> InitResponse {
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
fn focus_vault(id: i64, app_state: tauri::State<'_, AppState>, window: tauri::Window) {
    *app_state.vault_id.lock().unwrap() = id;
    let info = get_init_info(app_state);
    window.emit("current_vault_change", &info).unwrap();
}

#[tauri::command]
fn focus_project(id: Option<i64>, app_state: tauri::State<'_, AppState>, window: tauri::Window) {
    if let Some(id) = id {
        let db = app_state.database.lock().unwrap();
        db.update_project_timestamp(id);
    }

    let mut info = get_init_info(app_state);
    info.selected_id = id;
    window.emit("current_vault_change", &info).unwrap();
}

#[tauri::command]
fn reset_current_vault(app_state: tauri::State<'_, AppState>, window: tauri::Window) {
    focus_vault(1, app_state, window);
}

// When this function is called, the Settings are already initilized and only need to be updated
#[tauri::command]
fn set_vault_path(
    path: PathBuf,
    init_state: tauri::State<'_, InitState>,
    app_handle: tauri::AppHandle,
    window: tauri::Window,
) {
    let mut settings = init_state.settings.lock().unwrap().clone();
    settings.set_vault_path(path.clone());
    let path_resolver = app_handle.path_resolver();
    let config_path = path_resolver.app_config_dir().unwrap();
    settings.save(config_path);

    let app_state = setup_vault(path.as_path(), settings);
    app_handle.manage(app_state);

    window.emit("vault_set_success", {}).unwrap();
}

fn setup_vault(path: &Path, settings: Settings) -> AppState {
    let vault = Vault::top_level(path);
    let db = Db::new(path);
    db.fill_with_vault(&vault).unwrap();

    let app_state = AppState {
        database: Mutex::new(db),
        settings: Mutex::new(settings),
        vault_id: Mutex::new(1),
    };

    app_state
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn Error>> {
    let path_resolver = app.path_resolver();
    let config_path = path_resolver.app_config_dir().unwrap();
    let app_handle = app.app_handle();
    let settings = Settings::get_or_default(config_path.as_path());

    // If settings are set
    if let Some(path_buf) = settings.vault_path.clone() {
        let app_state = setup_vault(path_buf.as_path(), settings);
        app_handle.manage(app_state);
        let window_builder = get_desk(app);
        window_builder.build()?;
        return Ok(());
    }

    // If Settings are not set
    let init_state = InitState {
        settings: Mutex::new(settings),
    };
    app_handle.manage(init_state);
    let window_builder = get_welcome(app);
    window_builder.build()?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| setup(app))
        .invoke_handler(tauri::generate_handler![
            get_init_info,
            reset_current_vault,
            focus_vault,
            focus_project,
            set_vault_path,
            tauri_commands::terminal_at,
            tauri_commands::editor_at,
            tauri_commands::open,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
