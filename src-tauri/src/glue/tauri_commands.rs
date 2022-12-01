use std::path::Path;

use crate::core::commands;

#[tauri::command]
pub fn terminal_at(path: String, command_line: Option<String>) {
    commands::terminal_at(Path::new(&path), command_line);
}

#[tauri::command]
pub fn editor_at(path: String, command_line: Option<String>) {
    commands::editor_at(Path::new(&path), command_line);
}

#[tauri::command]
pub fn open(url: &Path) {
    open::that(url);
}
