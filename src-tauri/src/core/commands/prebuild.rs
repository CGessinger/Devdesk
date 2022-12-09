use std::fs;
use std::path::Path;

use super::arbitrary_command;
use super::defaults_library;

// ToDo make configurable
const DEFAULT_SCRIPTS: &[(&str, &str)] = &[
    ("workon.sh", defaults_library::DEFAULT_EDITOR_COMMAND),
    ("terminal.sh", defaults_library::DEFAULT_TERMINAL_COMMAND),
];

pub fn write_default_scripts(scripts_path: &Path) {
    for script in DEFAULT_SCRIPTS {
        let script_path = scripts_path.join(script.0);
        if !script_path.exists() {
            fs::write(script_path, script.1).unwrap();
        }
    }
}

pub async fn run_make(path: &Path) -> Result<Vec<String>, String> {
    arbitrary_command(&path, "make", [].into_iter())
}
