#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

mod files_api;
mod cmd_api;
mod db_api;
mod stats_api;
mod state_api;
mod settings_api;

fn main() {
  tauri::Builder::default()
    .manage(state_api::AppState::default())
    .invoke_handler(tauri::generate_handler![
      files_api::read_dir,
      files_api::read_file,
      files_api::load_image,
      files_api::write_image,
      files_api::folder_exists,
      files_api::file_exists,
      files_api::create_folder,
      files_api::write_to_file,
      files_api::makefile_exists,
      cmd_api::terminal_at,
      cmd_api::editor_at,
      cmd_api::git_clone,
      cmd_api::run_make,
      db_api::insert_project,
      db_api::insert_projects,
      db_api::get_projects_filter,
      db_api::get_projects,
      db_api::clear_db,
      stats_api::read_language_stats,
      settings_api::add_portfolio_to_settings,
      settings_api::remove_portfolio_from_settings,
      settings_api::update_portfolio_in_settings,
      settings_api::get_portfolios_from_settings,
      settings_api::get_switches_from_settings,
      settings_api::get_commands_from_settings,
      settings_api::set_settings
      ])
      .setup(|app| {
        let path_resolver = app.path_resolver();
        let app_dir = path_resolver.app_dir().ok_or("Could not find config dir".to_string())?;
        app.manage(settings_api::Settings::load(app_dir));
        Ok(())
      })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}
