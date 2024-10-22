use crate::{
    commands,
    config,
    plugin_proxy::proxy_server::ProxyServer
};
use std::net::{IpAddr, SocketAddr};
use tauri::{
    async_runtime,
    plugin::{Builder, TauriPlugin},
    Emitter, Runtime,
};

pub(crate) mod proxy_server;
pub(crate) mod proxy_handler;

pub(crate) fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("proxy")
        .invoke_handler(tauri::generate_handler![
            commands::proxy::start_proxy,
            commands::proxy::stop_proxy,
        ])
        .setup(move |app_handle, _api| {
            let app_handle = app_handle.clone();
            let (tx, mut rx) = async_runtime::channel::<proxy_handler::ProxyHandler>(100);
            async_runtime::spawn(async move {
                while let Some(message) = rx.recv().await {
                    app_handle.emit("proxy-event", message.to_parts()).unwrap();
                }
            });
            async_runtime::spawn(async move {
                let c = config::get_global_config();
                let addr = SocketAddr::from((c.address.parse::<IpAddr>().unwrap(), c.port));
                if let Err(e) = ProxyServer::new(addr, tx).start(shutdown_signal()).await {
                    eprintln!("Error running proxy on {:?}: {e}", addr);
                }
            });
            Ok(())
        })
        .build()
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}
