use std::{env, process::Command};

#[tauri::command]
pub fn terminal_at(path: String) -> Result<(), String> {
    match env::consts::OS {
        "macos" => {
            Command::new("open")
                .arg("-a")
                .arg("Terminal")
                .arg(path)
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        "windows" => {
            Command::new("cmd")
                .arg("/C")
                .arg("start")
                .arg("cmd")
                .arg("/K")
                .arg("cd")
                .arg(path)
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        "linux" => {
            Command::new("gnome-terminal")
                .arg("--working-directory")
                .arg(path)
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        _ => {
            return Err("Unsupported OS".to_string());
        }
    }
    Ok(())
}

#[tauri::command]
pub fn vscode_at(path: String) -> Result<(), String> {
    match env::consts::OS {
        "macos" => {
            Command::new("open")
                .arg("-a")
                .arg("Visual Studio Code")
                .arg(path)
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        "windows" => {
            Command::new("cmd")
                .arg("/C")
                .arg("start")
                .arg("code")
                .arg(path)
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        "linux" => {
            Command::new("code")
                .arg(path)
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        _ => {
            return Err("Unsupported OS".to_string());
        }
    }
    Ok(())
}
