#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use devdesk::api::{self, AppState};
use devdesk::app_windows::{get_desk, get_welcome};
use devdesk::core::database::Db;
use devdesk::core::filesystem::{configtree, runner};
use devdesk::core::settings::Settings;
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::async_runtime::Mutex;
use tauri::Manager;

// If settings are set
async fn setup(app_handle: &tauri::AppHandle, settings: Settings) {
    let path = settings.get_vault_path();
    let mut database = Db::new().await.unwrap();
    runner::walkdir_to_database(&mut database, &path, &settings.patterns).await;

    let app_state = AppState {
        settings: Mutex::new(settings),
        database: Mutex::new(database),
    };
    app_handle.manage(app_state);
    let window_builder = get_desk(&app_handle);
    window_builder.build().unwrap();
}

// If settings are not set
async fn prepare(app_handle: &tauri::AppHandle) {
    let window_builder = get_welcome(&app_handle);
    let window = window_builder.build().unwrap();

    let window_ = window.clone();
    window.once("pick_vault", move |_| {
        let path = FileDialogBuilder::new()
            .set_title("Select a folder")
            .pick_folder()
            .unwrap();

        let app_handle = window_.app_handle();
        let path_resolver = app_handle.path_resolver();
        let config_path = path_resolver.app_config_dir().unwrap();
        configtree::build_config_folders(&path);
        configtree::write_default_files(&path);

        let mut settings = Settings::default();
        settings.set_vault_path(&path);
        // Save updated settings
        settings.save(config_path);

        window_.close().ok();
        tauri::async_runtime::spawn(async move {
            setup(&app_handle, settings).await;
        });
    });
}

async fn init(app_handle: &tauri::AppHandle) {
    let path_resolver = app_handle.path_resolver();
    let config_path = path_resolver.app_config_dir().unwrap();

    let settings = Settings::try_get(config_path.as_path());

    let app_handle = app_handle.app_handle();
    if let Some(settings) = settings {
        setup(&app_handle, settings).await;
    } else {
        prepare(&app_handle).await;
    }
}

fn main() {
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.app_handle();
            tauri::async_runtime::spawn(async move {
                init(&app_handle).await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            api::get_all,
            api::focus_project,
            api::open,
            api::backup_vault,
            api::get_vault,
            api::read_directory,
            api::is_file,
            api::file_exists,
            api::read_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
