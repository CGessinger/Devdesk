use std::path::{Path};
use std::{fs};

#[tauri::command]
pub fn read_dir(path: String) -> Result<Vec<String>, String> {
  let path = Path::new(&path);
  let mut files = Vec::new();
  for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
    let entry = entry.map_err(|e| e.to_string())?;
    files.push(entry.file_name().to_string_lossy().to_string());
  }
  Ok(files)
}
