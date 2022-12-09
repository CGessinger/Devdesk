use std::{fs, path::Path};

use crate::core::filesystem;

use super::arbitrary_command;

pub fn read_custom_scripts(scripts_path: &Path) -> Vec<(String, String)> {
    let mut scripts = Vec::<(String, String)>::new();
    let dir_entries = fs::read_dir(scripts_path);
    if dir_entries.is_err() {
        return scripts;
    }

    for entry in dir_entries.unwrap() {
        if entry.is_err() {
            continue;
        }
        let script_path = entry.unwrap().path();
        if !script_path.is_file() {
            continue;
        }

        let name = filesystem::utils::name_from(script_path.as_path());
        let script = fs::read_to_string(script_path.as_path());
        if script.is_err() {
            continue;
        }

        scripts.push((name, script.unwrap()))
    }
    scripts
}

pub fn execute_custom_script(script: &str, path: &Path) -> Result<Vec<String>, String> {
    let mut parts = shellwords::split(script)
        .map_err(|e| e.to_string())?
        .into_iter();
    let command = parts.next().unwrap();
    let args = parts;

    arbitrary_command(&path, &command, args)
}
