use proxy::{data::ProxyData, server, store};
use std::net::{IpAddr, SocketAddr};
use tauri::{async_runtime, Emitter};
use tauri_plugin_log::{Target, TargetKind};

mod commands;
mod config;
mod error;
mod idgen;
mod menus;
mod proxy;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut ctx = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_theme::init(ctx.config_mut()))
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .setup(|app| {
            let app_handle = app.handle();
            let app_handle = app_handle.clone();
            let (tx, mut rx) = async_runtime::channel::<ProxyData>(100);
            async_runtime::spawn(async move {
                let mut store = store::ProxyDataStore::new();
                while let Some(proxy_data) = rx.recv().await {
                    match store.insert_or_update(proxy_data) {
                        Ok(data) => {
                            if let Err(e) = app_handle.emit("proxy-event", data) {
                                log::error!("Failed to emit proxy-event: {e}");
                            }
                        }
                        Err(e) => {
                            log::error!("Error inserting or updating proxy data: {e}");
                        }
                    }
                }
            });
            // start proxy server
            async_runtime::spawn(async move {
                let c = config::get_global_config();
                let addr = SocketAddr::from((c.address.parse::<IpAddr>().unwrap(), c.port));
                if let Err(e) = server::ProxyServer::new(addr, tx).start().await {
                    log::error!("Error running proxy on {:?}: {e}", addr);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::proxy::set_sys_proxy,
            commands::proxy::clean_sys_proxy,
            commands::proxy::get_proxy_status,
            commands::config::get_config,
        ])
        .run(ctx)
        .expect("error while running tauri application");
}
