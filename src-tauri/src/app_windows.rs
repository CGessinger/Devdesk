fn get_window<'a>(app: &'a tauri::App, url: &str) -> tauri::WindowBuilder<'a> {
    tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App(url.into()))
        .inner_size(800.0, 600.0)
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .hidden_title(true)
        .resizable(true)
}

pub fn get_desk(app: &tauri::App) -> tauri::WindowBuilder {
    get_window(app, "desk.html")
}

pub fn get_welcome(app: &tauri::App) -> tauri::WindowBuilder {
    get_window(app, "welcome.html")
}
