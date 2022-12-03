use std::{
    fs,
    path::{Path, PathBuf},
};

use super::Settings;

pub fn format_config_file_path(config_path: &Path) -> PathBuf {
    config_path.join("config").join("settings.json")
}
