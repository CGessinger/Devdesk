// ToDo make configurable
pub const DEFAULT_SCRIPTS: &[(&str, &str)] = &[
    ("workon.sh", DEFAULT_EDITOR_COMMAND),
    ("terminal.sh", DEFAULT_TERMINAL_COMMAND),
];

#[cfg(target_os = "linux")]
const DEFAULT_TERMINAL_COMMAND: &str = "gnome-terminal --working-directory .";
#[cfg(target_os = "macos")]
const DEFAULT_TERMINAL_COMMAND: &str = "open -a Warp .";
#[cfg(target_os = "windows")]
const DEFAULT_TERMINAL_COMMAND: &str = "cmd.exe /K \"cd .\"";

#[cfg(target_os = "linux")]
const DEFAULT_EDITOR_COMMAND: &str = "code .";
#[cfg(target_os = "macos")]
const DEFAULT_EDITOR_COMMAND: &str = "open -a \"Visual Studio Code\" .";
#[cfg(target_os = "windows")]
const DEFAULT_EDITOR_COMMAND: &str = "cmd /C start code .";
