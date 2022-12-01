use std::{path::Path, process::Command};

#[cfg(target_os = "linux")]
pub static DEFAULT_TERMINAL_COMMAND: &str = "gnome-terminal --working-directory .";
#[cfg(target_os = "macos")]
pub static DEFAULT_TERMINAL_COMMAND: &str = "open -a Warp .";
#[cfg(target_os = "windows")]
pub static DEFAULT_TERMINAL_COMMAND: &str = "cmd.exe /K \"cd .\"";

#[cfg(target_os = "linux")]
pub static DEFAULT_EDITOR_COMMAND: &str = "code .";
#[cfg(target_os = "macos")]
pub static DEFAULT_EDITOR_COMMAND: &str = "open -a \"Visual Studio Code\" .";
#[cfg(target_os = "windows")]
pub static DEFAULT_EDITOR_COMMAND: &str = "cmd /C start code .";

fn arbitrary_command<I>(path: &Path, command: &str, args: I) -> Result<Vec<String>, String>
where
    I: Iterator<Item = String>,
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

pub fn terminal_at(path: &Path, command_line: Option<String>) -> Result<Vec<String>, String> {
    let command_line = command_line.unwrap_or(DEFAULT_TERMINAL_COMMAND.to_string());
    let mut parts = shellwords::split(&command_line)
        .map_err(|e| e.to_string())?
        .into_iter();
    let command = parts.next().unwrap();
    let args = parts;

    arbitrary_command(&path, &command, args)
}

pub fn editor_at(path: &Path, command_line: Option<String>) -> Result<Vec<String>, String> {
    let command_line = command_line.unwrap_or(DEFAULT_EDITOR_COMMAND.to_string());
    let mut parts = shellwords::split(&command_line)
        .map_err(|e| e.to_string())?
        .into_iter();
    let command = parts.next().unwrap();
    let args = parts;

    arbitrary_command(&path, &command, args)
}

pub async fn run_make(path: &Path) -> Result<Vec<String>, String> {
    arbitrary_command(&path, "make", [].into_iter())
}
