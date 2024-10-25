use crate::error::RequeditError;
use crate::{config, proxy::handler::ProxyHandler};
use crate::utils;
use hudsucker::builder::ProxyBuilder;
use std::{future::Future, net::SocketAddr};
use tauri::async_runtime::Sender;

pub(crate) struct ProxyServer {
    addr: SocketAddr,
    tx: Sender<ProxyHandler>,
}

impl ProxyServer {
    pub(crate) fn new(addr: SocketAddr, tx: Sender<ProxyHandler>) -> Self {
        Self { addr, tx }
    }

    pub(crate) async fn start(
        self
    ) -> Result<(), RequeditError> {

        let addr = self.addr;
        let c = config::get_global_config();
        utils::generate_key_and_cer(&c.key_name, &c.cer_name);
        let ca = utils::get_ca(&c.key_name, &c.cer_name)?;

        // 使用 ProxyBuilder 创建代理服务器
        let proxy = ProxyBuilder::new()
            .with_addr(addr)
            .with_native_tls_client()
            .with_ca(ca)
            .with_http_handler(ProxyHandler::new(self.tx.clone()))
            .build();

        if let Err(e) = proxy.start(shutdown_signal()).await {
            println!("Failed to start the proxy server: {}", e);
        }
        Ok(())
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}
