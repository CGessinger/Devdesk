#[cfg(test)]
mod test {
    use std::{fs, path::Path};

    use crate::core::{
        database::Db,
        types::{Project, Vault},
    };

    fn ensure_empty_db<'a>() -> (Db, &'a Path) {
        let path = Path::new("/Users/cgessinger/Documents/Programming/maintained/devdesk/dev-test");
        fs::remove_file(path.join("reference")).unwrap();
        (Db::new(path), path)
    }

    #[test]
    fn db_creation() {
        let (_, path) = ensure_empty_db();
        assert!(path.join("reference").exists())
    }

    #[test]
    fn db_fill() {
        let (db, path) = ensure_empty_db();
        let vault = Vault::top_level(path);
        let fill_success = db.fill_with_vault(&vault);
        let sql = "SELECT * FROM Projects";
        assert!(fill_success.is_ok());
        let res: Vec<Project> = db.query_many(sql, []);
        assert_eq!(res.len(), 4);
    }

    #[test]
    fn under_vault() {
        let (db, path) = ensure_empty_db();
        let vault = Vault::top_level(path);
        let fill_success = db.fill_with_vault(&vault);
        assert!(fill_success.is_ok());
        let res: Vec<Project> = db.select_projects_under_vault(1);
        assert_eq!(res.len(), 4);
    }

    #[test]
    fn db_keep() {
        let (db, path) = ensure_empty_db();
        let vault = Vault::top_level(path);
        db.fill_with_vault(&vault).unwrap();
        let pbefore = db.select_project(1).unwrap();
        let db = Db::new(path);
        db.fill_with_vault(&vault).unwrap();
        let pafter = db.select_project(1).unwrap();
        assert_eq!(pbefore, pafter);
    }
}
