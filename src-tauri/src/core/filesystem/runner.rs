use async_recursion::async_recursion;
use std::fs::{self, DirEntry};
use std::path::Path;

use crate::core::database::Db;
use crate::core::models::NodeScheme;

use super::utils;

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn is_dir(entry: &DirEntry) -> bool {
    entry.file_type().unwrap().is_dir()
}

pub async fn walkdir_to_database(db: &mut Db, path: &Path, patterns: &Vec<String>) {
    walkdir(db, path, patterns, 0).await;
}

#[async_recursion]
async fn walkdir(db: &mut Db, path: &Path, patterns: &Vec<String>, depth: usize) {
    if depth > 3 {
        return;
    }

    let walk = fs::read_dir(path).unwrap();

    for entry in walk
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_dir(e) && !is_hidden(e))
    {
        let path = &entry.path();
        let is_project = utils::guess_is_project(path, patterns);
        let data = NodeScheme {
            name: entry.file_name().to_str().unwrap().to_string(),
            path: path.to_str().unwrap().to_string(),
            modified: utils::modified_from(path),
            depth: depth as i64,
            project: is_project,
        };
        db.insert(data).await.unwrap();

        if is_project {
            return;
        }
        walkdir(db, path, patterns, depth + 1).await;
    }
}
