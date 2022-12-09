#[cfg(target_os = "linux")]
pub const DEFAULT_TERMINAL_COMMAND: &str = "gnome-terminal --working-directory .";
#[cfg(target_os = "macos")]
pub const DEFAULT_TERMINAL_COMMAND: &str = "open -a Warp .";
#[cfg(target_os = "windows")]
pub const DEFAULT_TERMINAL_COMMAND: &str = "cmd.exe /K \"cd .\"";

#[cfg(target_os = "linux")]
pub const DEFAULT_EDITOR_COMMAND: &str = "code .";
#[cfg(target_os = "macos")]
pub const DEFAULT_EDITOR_COMMAND: &str = "open -a \"Visual Studio Code\" .";
#[cfg(target_os = "windows")]
pub const DEFAULT_EDITOR_COMMAND: &str = "cmd /C start code .";
