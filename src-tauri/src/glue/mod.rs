use crate::core::types::{Project, Vault};

pub mod tauri_commands;

#[derive(serde::Serialize)]
pub struct DasboardResponse {
    pub vault: Vault,
    pub sub_directories: Vec<Vault>,
    pub projects: Vec<Project>,
    pub recent: Vec<Project>,
    pub selected: Option<Project>,
}
