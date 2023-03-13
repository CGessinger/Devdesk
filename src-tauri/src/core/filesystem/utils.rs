use chrono::prelude::{DateTime, Utc};
use std::{
    fs,
    io::{Read, Seek, Write},
    path::Path,
};
use walkdir::WalkDir;

pub fn is_file(file: &fs::DirEntry) -> bool {
    let meta = file.metadata();
    match meta {
        Ok(m) => m.is_file(),
        _ => false,
    }
}

pub fn modified_from(file: &fs::DirEntry) -> String {
    if let Ok(meta) = file.metadata() {
        if let Ok(modified) = meta.modified() {
            let dt: DateTime<Utc> = modified.into();
            return format!("{}", dt.format("%Y-%m-%d"));
        }
    }
    return String::from("Idk");
}

pub fn name_from(file_path: &Path) -> String {
    file_path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn guess_is_project(folder: &fs::DirEntry, indicators: &Vec<String>) -> bool {
    let path = folder.path();
    let dir_entries = fs::read_dir(path);
    if dir_entries.is_err() {
        return false;
    }

    for entry in dir_entries.unwrap() {
        if entry.is_err() {
            continue;
        }
        let file = entry.unwrap();
        if is_file(&file) {
            let name = name_from(&file.path());
            if indicators.contains(&name) {
                return true;
            }
        }
    }
    return false;
}

pub fn read_readme(project_path: &Path) -> String {
    let readme_path = project_path.join("README.md");
    if !readme_path.exists() {
        return "".to_string();
    }
    let readme = fs::read_to_string(readme_path);
    if readme.is_err() {
        return "".to_string();
    }
    return readme.unwrap();
}

pub fn backup_vault<F: Fn(f64)>(vault_path: &Path, backup_path: &Path, progress_callback: F) {
    let date = chrono::offset::Local::now();
    let vault_name = vault_path.file_name().unwrap().to_str().unwrap();
    let backup_file_name = format!("{}-{}.zip", vault_name, date.format("%Y-%m-%dT%H%M%S")); // ISO 8601
    let backup_path = backup_path.join(backup_file_name);
    let file = fs::File::create(backup_path).unwrap();

    let walkdir = WalkDir::new(vault_path);
    let it = walkdir.into_iter();

    zip_dir(
        &mut it.filter_map(|e| e.ok()),
        vault_path.to_str().unwrap(),
        file,
        progress_callback,
    )
    .unwrap();
}

fn zip_dir<T, F>(
    it: &mut dyn Iterator<Item = walkdir::DirEntry>,
    prefix: &str,
    writer: T,
    progress_callback: F,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
    F: Fn(f64),
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = zip::write::FileOptions::default().unix_permissions(0o755);

    let mut buffer = Vec::new();
    let mut bytes_zipped = 0;
    let twenty_megabytes = 20 * 1e6 as u64;
    let mut bytes_stepped = twenty_megabytes;

    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        if path.is_file() {
            zip.start_file(name.to_str().unwrap(), options)?;
            let mut f = fs::File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            bytes_zipped += f.metadata().unwrap().len();
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(name.to_str().unwrap(), options)?;
        }

        if bytes_zipped > bytes_stepped {
            progress_callback(bytes_stepped as f64 * 1e-9 as f64);
            bytes_stepped += twenty_megabytes;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

pub fn calculate_vault_size(path: &Path) -> f64 {
    let mut size = 0;
    let walkdir = WalkDir::new(path);
    let it = walkdir.into_iter();
    for entry in it.filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            size += entry.metadata().unwrap().len();
        }
    }
    return (size as f64) * 1e-9; // in GB
}
