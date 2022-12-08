use std::fs;
use std::path::Path;

use super::arbitrary_command;
use super::defaults;

pub fn write_default_scripts(scripts_path: &Path) {
    let workon_path = scripts_path.join("workon.sh");
    if !workon_path.exists() {
        fs::write(workon_path, defaults::DEFAULT_EDITOR_COMMAND).unwrap();
    }
    let terminal_path = scripts_path.join("terminal.sh");
    if !terminal_path.exists() {
        fs::write(terminal_path, defaults::DEFAULT_TERMINAL_COMMAND).unwrap();
    }
}

pub async fn run_make(path: &Path) -> Result<Vec<String>, String> {
    arbitrary_command(&path, "make", [].into_iter())
}
