use crate::core::types::{Project, Vault};

pub mod tauri_commands;

#[derive(serde::Serialize)]
pub struct InitResponse {
    pub vault: Vault,
    pub sub_directories: Vec<Vault>,
    pub projects: Vec<Project>,
    pub recent: Vec<Project>,
    pub selected_id: Option<i64>,
}

#[derive(serde::Serialize)]
pub struct ViewResponse {
    pub readme: String,
}
