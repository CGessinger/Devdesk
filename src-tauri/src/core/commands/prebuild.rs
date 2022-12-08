use std::path::Path;

use super::arbitrary_command;
use super::defaults;

pub fn terminal_at(path: &Path, command_line: Option<String>) -> Result<Vec<String>, String> {
    let command_line = command_line.unwrap_or(defaults::DEFAULT_TERMINAL_COMMAND.to_string());
    let mut parts = shellwords::split(&command_line)
        .map_err(|e| e.to_string())?
        .into_iter();
    let command = parts.next().unwrap();
    let args = parts;

    arbitrary_command(&path, &command, args)
}

pub fn editor_at(path: &Path, command_line: Option<String>) -> Result<Vec<String>, String> {
    let command_line = command_line.unwrap_or(defaults::DEFAULT_EDITOR_COMMAND.to_string());
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
