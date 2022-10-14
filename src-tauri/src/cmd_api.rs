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

#[tauri::command]
pub fn git_clone(url: String, branch: String, path: String) -> Result<(), String> {
    Command::new("git")
        .current_dir(&path)
        .arg("init")
        .spawn()
        .map_err(|e| e.to_string())?;
    Command::new("git")
        .current_dir(&path)
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(url)
        .spawn()
        .map_err(|e| e.to_string())?;
    Command::new("git")
        .current_dir(&path)
        .arg("pull")
        .arg("origin")
        .arg(branch)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn run_make(path: String) -> Result<(), String> {
    Command::new("make")
        .current_dir(&path)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}
