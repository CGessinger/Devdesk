use std::path::{Path};
use std::{fs, io};
use image::io::Reader as ImageReader;

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

#[tauri::command]
pub fn load_image(path: String) -> Result<String, String> {
  let path = Path::new(&path);
  let _img = ImageReader::open(path).map_err(|e| e.to_string())?;
  let img = _img.decode().map_err(|e| e.to_string())?;
  let mut buf = Vec::new();
  img.write_to(&mut io::Cursor::new(&mut buf), image::ImageOutputFormat::Png).map_err(|e| e.to_string())?;
  Ok(base64::encode(&buf))
}

#[tauri::command]
pub fn folder_exists(path: String) -> Result<bool, String> {
  let path = Path::new(&path);
  Ok(path.exists() && path.is_dir())
}

#[tauri::command]
pub fn create_folder(path: String) -> Result<(), String> {
  let path = Path::new(&path);
  fs::create_dir_all(path).map_err(|e| e.to_string())?;
  Ok(())
}
