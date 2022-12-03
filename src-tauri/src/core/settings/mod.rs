use std::{
    fs,
    path::{Path, PathBuf},
};

pub mod utils;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Settings {
    pub vault_path: Option<PathBuf>,
}
impl Default for Settings {
    fn default() -> Self {
        Self { vault_path: None }
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
        println!("{:?}", file_path);
        fs::write(&file_path, serialized).unwrap();
    }

    pub fn get_or_default(config_path: &Path) -> Self {
        let config_file_path = utils::format_config_file_path(config_path);
        let file = fs::read_to_string(config_file_path);
        match file {
            Ok(content) => Settings::deserialize(content),
            _ => Settings::default(),
        }
    }

    pub fn set_vault_path(&mut self, vault_path: PathBuf) {
        self.vault_path = Some(vault_path);
    }
}
