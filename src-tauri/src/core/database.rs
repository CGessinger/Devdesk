use sqlx::{Connection, SqliteConnection};

use super::models::{Node, NodeScheme};

pub struct Db(sqlx::SqliteConnection);
impl Db {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let mut connection = SqliteConnection::connect("sqlite::memory:").await?;
        sqlx::query(
            r#"CREATE TABLE "node" (
                "id"	INTEGER NOT NULL UNIQUE,
                "name"	TEXT NOT NULL,
                "path"	TEXT NOT NULL UNIQUE,
                "modified"	TEXT NOT NULL,
                "depth"	INTEGER NOT NULL,
                "project"	BOOLEAN,
                PRIMARY KEY("id" AUTOINCREMENT)
            )"#,
        )
        .execute(&mut connection)
        .await?;
        Ok(Self(connection))
    }

    pub async fn insert(&mut self, data: NodeScheme) -> Result<i64, sqlx::Error> {
        let id = sqlx::query!(
            r#"
    INSERT INTO node ( name, path, modified, depth, project )
    VALUES ( ?1, ?2, ?3, ?4, ?5 )
            "#,
            data.name,
            data.path,
            data.modified,
            data.depth,
            data.project
        )
        .execute(&mut self.0)
        .await?
        .last_insert_rowid(); // Keep this in case I need it

        Ok(id)
    }

    pub async fn get_all(&mut self) -> Result<Vec<Node>, sqlx::Error> {
        let nodes = sqlx::query_as!(
            Node,
            r#"
    SELECT * FROM node
            "#
        )
        .fetch_all(&mut self.0)
        .await?;

        Ok(nodes)
    }

    pub async fn get_by_path(&mut self, path: String) -> Result<Node, sqlx::Error> {
        let node = sqlx::query_as!(
            Node,
            r#"
    SELECT * FROM node WHERE path = ?1
            "#,
            path
        )
        .fetch_one(&mut self.0)
        .await?;

        Ok(node)
    }

    pub async fn get_by_id(&mut self, id: i64) -> Result<Node, sqlx::Error> {
        let node = sqlx::query_as!(
            Node,
            r#"
    SELECT * FROM node WHERE id = ?1
            "#,
            id
        )
        .fetch_one(&mut self.0)
        .await?;

        Ok(node)
    }
}
