mod commands;
mod config;
mod error;
mod menus;
mod plugin_proxy;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut ctx = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_theme::init(ctx.config_mut()))
        .plugin(plugin_proxy::init())
        .run(ctx)
        .expect("error while running tauri application");
}
