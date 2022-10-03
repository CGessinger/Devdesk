#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod files_api;
mod cmd_api;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      files_api::read_dir,
      files_api::read_file,
      files_api::load_image,
      files_api::write_image,
      files_api::folder_exists,
      files_api::file_exists,
      files_api::create_folder,
      files_api::write_to_file,
      cmd_api::terminal_at,
      cmd_api::vscode_at,
      cmd_api::git_clone,
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}
