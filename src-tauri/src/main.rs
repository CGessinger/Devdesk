#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Manager, WindowBuilder};

mod files_api;
mod cmd_api;
mod db_api;
mod stats_api;
mod settings_api;

fn main() {
  tauri::Builder::default()
    .manage(stats_api::Languages::new())
    .manage(db_api::Database::new())
    .invoke_handler(tauri::generate_handler![
      files_api::read_dir,
      files_api::read_dirs_in_dir,
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
      db_api::load_recursive,
      db_api::query_database,
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
        tauri::WindowBuilder::new(
          app, "label", 
          tauri::WindowUrl::App("index.html".into()))
            .title("Project Manager")
            .hidden_title(true)
            .title_bar_style(tauri::TitleBarStyle::Overlay)
            .build()
            .unwrap();
        
        let path_resolver = app.path_resolver();
        let app_dir = path_resolver.app_dir().ok_or("Could not find config dir".to_string())?;
        app.manage(settings_api::Settings::load(app_dir));
        Ok(())
      })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}
