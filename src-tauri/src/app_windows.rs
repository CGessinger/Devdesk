fn get_window<'a>(app: &'a tauri::AppHandle, url: &str, label: &str) -> tauri::WindowBuilder<'a> {
    let window = tauri::WindowBuilder::new(app, label, tauri::WindowUrl::App(url.into()))
        .title("DevDesk")
        .inner_size(800.0, 600.0)
        .resizable(true);

    #[cfg(target_os = "macos")]
    return window
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .hidden_title(true);

    return window;
}

pub fn get_desk(app: &tauri::AppHandle) -> tauri::WindowBuilder {
    get_window(app, "desk.html", "desk")
}

pub fn get_welcome(app: &tauri::AppHandle) -> tauri::WindowBuilder {
    get_window(app, "welcome.html", "welcome")
}
