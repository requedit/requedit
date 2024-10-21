use crate::{config, mitm::proxy_handler::ProxyHandler};
use crate::utils;
use hudsucker::builder::ProxyBuilder;
use std::{fmt, future::Future, net::SocketAddr};
use tauri::async_runtime::Sender;

pub(crate) struct Proxy {
    addr: SocketAddr,
    tx: Sender<ProxyHandler>,
}

impl Proxy {
    pub(crate) fn new(addr: SocketAddr, tx: Sender<ProxyHandler>) -> Self {
        Self { addr, tx }
    }

    pub(crate) async fn start<F: Future<Output = ()> + Send + 'static>(
        self,
        signal: F,
    ) -> Result<(), MyError> {
        // 读取自签名证书

        let addr = self.addr;
        let c = config::get_global_config();
        utils::generate_key_and_cer(&c.key_name, &c.cer_name);
        let ca = utils::get_ca(&c.key_name, &c.cer_name).unwrap();

        // 使用 ProxyBuilder 创建代理服务器
        let proxy = ProxyBuilder::new()
            .with_addr(addr)
            .with_native_tls_client()
            .with_ca(ca)
            .with_http_handler(ProxyHandler::new(self.tx.clone()))
            .build();

        if let Err(e) = proxy.start(signal).await {
            println!("Failed to start the proxy server: {}", e);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) enum MyError {
    Hudsucker(hudsucker::Error),
    Hyper(hyper::Error),
    Other(String),
}

impl From<hudsucker::Error> for MyError {
    fn from(err: hudsucker::Error) -> Self {
        MyError::Hudsucker(err)
    }
}

impl From<hyper::Error> for MyError {
    fn from(err: hyper::Error) -> Self {
        MyError::Hyper(err)
    }
}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Hyper(e) => write!(f, "Hyper error: {}", e),
            MyError::Hudsucker(e) => write!(f, "Hudsucker error: {}", e),
            MyError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}
impl std::error::Error for MyError {}
