fn get_window<'a>(app: &'a tauri::AppHandle, url: &str, label: &str) -> tauri::WindowBuilder<'a> {
    tauri::WindowBuilder::new(app, label, tauri::WindowUrl::App(url.into()))
        .inner_size(800.0, 600.0)
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .hidden_title(true)
        .resizable(true)
}

pub fn get_desk(app: &tauri::AppHandle) -> tauri::WindowBuilder {
    get_window(app, "desk.html", "desk")
}

pub fn get_welcome(app: &tauri::AppHandle) -> tauri::WindowBuilder {
    get_window(app, "welcome.html", "welcome")
}
