use std::net::IpAddr;
use std::net::SocketAddr;
use tauri::async_runtime;
use tauri::Emitter;
use proxy::{handler, server};

mod commands;
mod config;
mod error;
mod menus;
mod proxy;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut ctx = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_theme::init(ctx.config_mut()))
        .setup(|app| {
            let app_handle = app.handle();
            let app_handle = app_handle.clone();
            let (tx, mut rx) = async_runtime::channel::<handler::ProxyHandler>(100);
            async_runtime::spawn(async move {
                while let Some(message) = rx.recv().await {
                    app_handle.emit("proxy-event", message.to_parts()).unwrap();
                }
            });
            async_runtime::spawn(async move {
                let c = config::get_global_config();
                let addr = SocketAddr::from((c.address.parse::<IpAddr>().unwrap(), c.port));
                if let Err(e) = server::ProxyServer::new(addr, tx).start().await {
                    eprintln!("Error running proxy on {:?}: {e}", addr);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::proxy::set_sys_proxy,
            commands::proxy::clean_sys_proxy,
        ])
        .run(ctx)
        .expect("error while running tauri application");
}
