use std::process::Command;

#[cfg(target_os = "linux")]
pub static DEFAULT_TERMINAL_COMMAND: &str = "gnome-terminal --working-directory .";
#[cfg(target_os = "macos")]
pub static DEFAULT_TERMINAL_COMMAND: &str = "open -a Terminal .";
#[cfg(target_os = "windows")]
pub static DEFAULT_TERMINAL_COMMAND: &str = "cmd.exe /K \"cd .\"";

#[cfg(target_os = "linux")]
pub static DEFAULT_EDITOR_COMMAND: &str = "code .";
#[cfg(target_os = "macos")]
pub static DEFAULT_EDITOR_COMMAND: &str = "open -a \"Visual Studio Code\" .";
#[cfg(target_os = "windows")]
pub static DEFAULT_TEDITORCOMMAND: &str = "cmd /C start code .";


fn arbitrary_command<I>(path: &str, command: &str, args: I) -> Result<Vec<String>, String> 
    where I: Iterator<Item = String> 
{
    let child = Command::new(command)
                    .current_dir(path)
                    .args(args)
                    .output()
                    .map_err(|e| e.to_string())?;

    let stdout = std::str::from_utf8(&child.stdout).map_err(|e| e.to_string())?;
    let stderr = std::str::from_utf8(&child.stderr).map_err(|e| e.to_string())?;
    Ok(Vec::from([stdout.to_string(), stderr.to_string()]))
}

#[tauri::command]
pub fn terminal_at(path: String, command_line: String) -> Result<Vec<String>, String> {
    let mut parts = shellwords::split(&command_line)
        .map_err(|e| e.to_string())?
        .into_iter();
    let command = parts.next().unwrap();
    let args = parts;

    arbitrary_command(&path, &command, args)
}

#[tauri::command]
pub fn editor_at( path: String, command_line: String) -> Result<Vec<String>, String> {
    let mut parts = shellwords::split(&command_line)
        .map_err(|e| e.to_string())?
        .into_iter();
    let command = parts.next().unwrap();
    let args = parts;

    arbitrary_command(&path, &command, args)
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
pub async fn run_make(path: String) -> Result<Vec<String>, String> {
    arbitrary_command(&path, "make", [].into_iter())
}
