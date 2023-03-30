use std::fs;
use std::path::{Path, PathBuf};

pub mod utils;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Settings {
    pub vault_path: PathBuf,
    pub patterns: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            vault_path: PathBuf::from(""),
            patterns: vec![
                String::from("*src"),
                String::from("*.prj"),
                String::from("*lib"),
                String::from("*bin"),
                String::from("*Cargo.toml"),
                String::from("*package.json"),
                String::from("*.git"),
                String::from("*.gitignore"),
                String::from("*main.py"),
            ],
        }
    }
}

impl Settings {
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn deserialize(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }

    pub fn save(&self, config_path: PathBuf) {
        // ToDo handle errors
        let file_path = utils::format_config_file_path(config_path.as_path());
        let serialized = self.serialize();
        fs::create_dir_all(&file_path.parent().unwrap()).unwrap();
        fs::write(&file_path, serialized).unwrap();
    }

    pub fn try_get(config_path: &Path) -> Option<Self> {
        let config_file_path = utils::format_config_file_path(config_path);
        let file = fs::read_to_string(config_file_path);
        match file {
            Ok(content) => Some(Settings::deserialize(content)),
            _ => None,
        }
    }

    pub fn set_vault_path(&mut self, vault_path: &Path) {
        self.vault_path = vault_path.to_path_buf();
    }

    pub fn get_vault_path(&self) -> PathBuf {
        self.vault_path.clone()
    }
}
