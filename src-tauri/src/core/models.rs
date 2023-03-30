#[derive(serde::Serialize, sqlx::FromRow, Debug)]
pub struct Node {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub modified: String,
    pub depth: i64,
    pub project: bool,
}

pub struct NodeScheme {
    pub name: String,
    pub path: String,
    pub modified: String,
    pub project: bool,
    pub depth: i64,
}
