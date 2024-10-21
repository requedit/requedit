// use tauri::SystemTray;
// use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

// pub fn create_system_tray() -> SystemTray {
//     let quit = CustomMenuItem::new("quit".to_string(), "Quit");
//     let hide = CustomMenuItem::new("hide".to_string(), "Hide");
//     let tray_menu = SystemTrayMenu::new()
//         .add_item(quit)
//         .add_native_item(SystemTrayMenuItem::Separator)
//         .add_item(hide);
//     let system_tray = SystemTray::new().with_menu(tray_menu);
//     system_tray
// }
