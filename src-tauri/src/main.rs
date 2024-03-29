#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use devdesk::app_windows::{get_desk, get_welcome};
use devdesk::core::settings::Settings;
use devdesk::glue::tauri_commands;
use devdesk::state::AppState;
use std::error::Error;
use tauri::Manager;

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn Error>> {
    let path_resolver = app.path_resolver();
    let config_path = path_resolver.app_config_dir().unwrap();
    let app_handle = app.app_handle();

    let settings = Settings::get_or_default(config_path.as_path());

    // If settings are set
    if let Some(vault_path_buf) = settings.vault_path.clone() {
        let app_state = AppState::new(vault_path_buf.as_path());
        app_handle.manage(app_state);
        let window_builder = get_desk(&app_handle);
        window_builder.build()?;
        return Ok(());
    }

    let window_builder = get_welcome(&app_handle);
    window_builder.build()?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| setup(app))
        .invoke_handler(tauri::generate_handler![
            tauri_commands::pick_vault,
            tauri_commands::get_project_view,
            tauri_commands::get_init_info,
            tauri_commands::reset_current_vault,
            tauri_commands::focus_vault,
            tauri_commands::focus_project,
            tauri_commands::open,
            tauri_commands::execute_script_by_name,
            tauri_commands::backup_vault,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
