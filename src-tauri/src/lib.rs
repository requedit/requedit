mod commands;
mod menus;
mod mitm;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(mitm::init())
        .setup(move |_| {
            commands::proxy::start_proxy();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::proxy::start_proxy,
            commands::proxy::stop_proxy,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
