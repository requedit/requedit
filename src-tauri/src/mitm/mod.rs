use std::net::SocketAddr;
use tauri::{
    async_runtime,
    plugin::{Builder, TauriPlugin},
    Emitter, Runtime,
};

pub(crate) mod proxy;
pub(crate) mod proxy_handler;

pub(crate) fn init<R: Runtime>() -> TauriPlugin<R> {
    // 创建广播频道
    let (tx, mut rx) = async_runtime::channel::<proxy_handler::ProxyHandler>(100);
    Builder::new("proxy")
        .setup(|app_handle, api| {
            let app_handle = app_handle.clone();
            // 异步任务，接收来自代理的请求并通过 Tauri 的事件系统发送给前端
            async_runtime::spawn(async move {
                while let Some(message) = rx.recv().await {
                    println!("Received message: {:?}", message);
                    app_handle
                        .emit("proxy-event", message.to_parts())
                        .unwrap();
                }
            });
            async_runtime::spawn(async move {
                let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
                if let Err(e) = proxy::Proxy::new(addr, tx).start(shutdown_signal()).await {
                    eprintln!("Error running proxy on {:?}: {e}", addr);
                }

                println!("Tauri app setup complete.");
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
