use std::{path::PathBuf, collections::HashMap};

use serde_json::Value;

use crate::{files_api as fs, state_api::AppState};

pub const LAN_JSON: &str = include_str!("languages.json");

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct LanguageStats {
    total: f64,
    languages: HashMap<String, f64>
}

#[tauri::command]
pub fn read_language_stats (state: tauri::State<'_, AppState>, path: String) -> Result<LanguageStats, String> {
    let languages = &state.languages;
    read_language_stats_nostate(path, languages)
}

pub fn read_language_stats_nostate(path: String, compare_list: &HashMap<String, Vec<String>>) -> Result<LanguageStats, String> {
    let mut total = 0f64;
    let mut languages = HashMap::<String, f64>::new();

    let files = fs::read_dir(path.to_string())?;
    for file in files {
        let mut buf = PathBuf::from(&path);
        buf.push(&file);
        if buf.is_dir() {
            let stats = read_language_stats_nostate(buf.to_str().unwrap().to_string(), compare_list)?;
            total += stats.total;
            for (key, val) in stats.languages.iter() {
                let value = val + *languages.get(key).unwrap_or(&0f64);
                languages.insert(key.to_owned(), value);
            }
        } else if buf.is_file() {
            let ext = buf.extension().and_then(std::ffi::OsStr::to_str).unwrap_or("").to_owned();
            let extension_opt = compare_list.get(&ext);
            if extension_opt.is_none() {
                continue;
            }
            let extension = extension_opt.unwrap()[0].to_string();
            let meta = buf.metadata().map_err(|e| e.to_string())?;
            let size = meta.len() as f64 / 1000000f64; // Convert Byte to Megabyte
            total += size;
            let value = size + *languages.get(&extension).unwrap_or(&0f64);
            languages.insert(extension, value);
        }
    }

    Ok(LanguageStats { 
        total, 
        languages 
    })
}