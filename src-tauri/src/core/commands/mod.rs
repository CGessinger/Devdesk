use std::{path::Path, process::Command};

pub mod custom;
mod defaults_library;
pub mod prebuild;

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

#[cfg(test)]
mod tests;
