use tauri::{
    menu::{MenuBuilder, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

use crate::commands;

pub fn create(handle: &tauri::AppHandle) {
    let start = MenuItem::with_id(handle, "start", "Start", true, None::<&str>).unwrap();
    let stop = MenuItem::with_id(handle, "stop", "Stop", true, None::<&str>).unwrap();
    let show = MenuItem::with_id(handle, "show", "Show", true, None::<&str>).unwrap();
    let menu = MenuBuilder::new(handle)
        .items(&[&start, &stop, &show])
        .separator()
        .text("quit", "Quit")
        .build()
        .unwrap();

    let _ = TrayIconBuilder::new()
        .icon(handle.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                println!("left click pressed and released");
                // in this example, let's show and focus the main window when the tray is clicked
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {
                println!("unhandled event {event:?}");
            }
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "start" => {
                log::info!("start menu item was clicked");
                commands::proxy::set_sys_proxy(app.clone());
            }
            "stop" => {
                log::info!("stop menu item was clicked");
                commands::proxy::clean_sys_proxy(app.clone());
            }
            "show" => {
                log::info!("show menu item was clicked");
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                log::info!("quit menu item was clicked");
                commands::proxy::clean_sys_proxy(app.clone());
                app.exit(0);
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .menu(&menu)
        .menu_on_left_click(true)
        .build(handle);
}
