use std::path::PathBuf;
use std::sync::Mutex;

use crate::files_api;
use crate::cmd_api;

#[allow(non_snake_case)]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Portfolio {
    pub path: String,
    #[serde(default)]
    pub subDirFilter: Vec<String>
}

impl PartialEq for Portfolio {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }

    fn ne(&self, other: &Self) -> bool {
        self.path != other.path
    }
}

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Switches {
    darkMode: bool,
    runThree: bool,
    experimental: bool,
}

impl Default for Switches {
    fn default() -> Self {
        Self {
            darkMode: false,
            runThree: false,
            experimental: false,
        }
    }
}

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Commands {
    editorCmd: String,
    terminalCmd: String,
}

impl Default for Commands {
    fn default() -> Self {
        Self {
            editorCmd: cmd_api::DEFAULT_EDITOR_COMMAND.to_string(),
            terminalCmd: cmd_api::DEFAULT_TERMINAL_COMMAND.to_string(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Settings {
    portfolios: Mutex<Vec<Portfolio>>,
    switches: Mutex<Switches>,
    commands: Mutex<Commands>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            portfolios: Mutex::new(vec![]),
            switches: Mutex::new(Switches::default()),
            commands: Mutex::new(Commands::default()),
        }
    }
}

impl Settings {
    fn add_portfolio(&self, portfolio: Portfolio) -> Result<Vec<Portfolio>, String> {
        let mutex = &self.portfolios;
        let mut guard = mutex.lock().map_err(|e| e.to_string())?;
        if !guard.contains(&portfolio) {
            guard.push(portfolio);
        }
        println!("Portfolios: {:?}", guard);
        Ok(guard.clone())
    }

    fn remove_portfolio(&self, path: String) -> Result<Vec<Portfolio>, String> {
        let mutex = &self.portfolios;
        let mut guard = mutex.lock().map_err(|e| e.to_string())?;
        guard.retain(|p| p.path != path);
        Ok(guard.clone())
    }

    fn update_portfolio(&self, portfolio: Portfolio) -> Result<Vec<Portfolio>, String> {
        let mutex = &self.portfolios;
        let mut guard = mutex.lock().map_err(|e| e.to_string())?;
        let mut iterator = guard.iter();
        iterator.position(|p| p.path == portfolio.path).map(|i| guard[i] = portfolio);
        Ok(guard.clone())
    }

    fn set(&self, switches: Switches, commands: Commands) -> Result<(), String> {
        let switches_mutex = &self.switches;
        let commands_mutex = &self.commands;
        let mut switches_guard = switches_mutex.lock().map_err(|e| e.to_string())?;
        let mut commands_guard = commands_mutex.lock().map_err(|e| e.to_string())?;
        *switches_guard = switches;
        *commands_guard = commands;
        Ok(())
    }

    fn save(&self, app_dir: PathBuf) -> Result<(), String> {
        let config_path = app_dir.join("config");
        files_api::create_folder(config_path.to_str().unwrap().to_string())?;
        let settings_path = config_path.join("settings.json");
        let settings_json = serde_json::to_string_pretty(&self).map_err(|e| e.to_string())?;
        files_api::write_to_file(settings_path.to_str().unwrap().to_string(), settings_json)
    }

    pub fn load(app_dir: PathBuf) -> Settings {
        let settings_path = app_dir.join("config").join("settings.json");
        if let Ok(settings_json) = files_api::read_file(settings_path.to_str().unwrap().to_string()) {
            let result = 
                serde_json::from_str(&settings_json)
                .unwrap_or(Settings::default());
            result
        } else {
            Settings::default()
        }
    }
}

#[tauri::command]
pub async fn add_portfolio_to_settings(
    settings: tauri::State<'_, Settings>, 
    app_handle: tauri::AppHandle, 
    portfolio: Portfolio) -> Result<Vec<Portfolio>, String> 
{
    let portfolios: Vec<Portfolio> = settings.add_portfolio(portfolio)?;

    let path_resolver = app_handle.path_resolver();
    let app_dir = path_resolver.app_dir().ok_or("Could not find config dir".to_string())?;
    settings.save(app_dir)?;
    Ok(portfolios)
}

#[tauri::command]
pub fn remove_portfolio_from_settings(
    settings: tauri::State<Settings>, 
    app_handle: tauri::AppHandle, 
    path: String) -> Result<Vec<Portfolio>, String> 
{
    let portfolios: Vec<Portfolio> = settings.remove_portfolio(path)?;

    let path_resolver = app_handle.path_resolver();
    let app_dir = path_resolver.app_dir().ok_or("Could not find config dir".to_string())?;
    settings.save(app_dir)?;
    Ok(portfolios)
}

#[tauri::command]
pub fn update_portfolio_in_settings(
    settings: tauri::State<Settings>, 
    app_handle: tauri::AppHandle, 
    portfolio: Portfolio) -> Result<Vec<Portfolio>, String> 
{
    let portfolios: Vec<Portfolio> = settings.update_portfolio(portfolio)?;

    let path_resolver = app_handle.path_resolver();
    let app_dir = path_resolver.app_dir().ok_or("Could not find config dir".to_string())?;
    settings.save(app_dir)?;
    Ok(portfolios)
}

#[tauri::command]
pub fn get_portfolios_from_settings(settings: tauri::State<Settings>) -> Result<Vec<Portfolio>, String> {
    let p_list = settings.portfolios.lock().map_err(|e| e.to_string())?;
    Ok(p_list.clone())
}

#[tauri::command]
pub fn get_switches_from_settings (settings: tauri::State<'_, Settings>) -> Switches {
    let switches = settings.switches.lock().unwrap();
    switches.clone()
}

#[tauri::command]
pub fn get_commands_from_settings (settings: tauri::State<'_, Settings>) -> Commands {
    let commands = settings.commands.lock().unwrap();
    commands.clone()
}

#[allow(non_snake_case)]
#[derive(serde::Serialize)]
pub struct SettingsJsFormat {
    switches: Switches,
    commands: Commands,
}
#[tauri::command]
pub async fn set_settings (
    settings: tauri::State<'_, Settings>,
    app_handle: tauri::AppHandle,
    switches: Switches, 
    commands: Commands) -> Result<SettingsJsFormat, String>
{
    settings.set(switches.clone(), commands.clone())?;
    
    let path_resolver = app_handle.path_resolver();
    let app_dir = path_resolver.app_dir().ok_or("Could not find config dir".to_string())?;
    settings.save(app_dir)?;

    Ok(SettingsJsFormat {
        switches,
        commands,
    })
}
