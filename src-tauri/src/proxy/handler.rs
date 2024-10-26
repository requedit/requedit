use std::{hash::{DefaultHasher, Hash, Hasher}, net::SocketAddr, time::SystemTime};

use async_trait::async_trait;
use hudsucker::{
    hyper::{Body, Method, Request, Response},
    HttpContext, HttpHandler, RequestOrResponse,
};
use tauri::async_runtime::Sender;
use uuid::Uuid;

use crate::proxy::data::ProxyData;



#[derive(Clone, Debug)]
pub(crate) struct ProxyHandler {
    tx: Sender<ProxyData>,
    req_id: Uuid,
}

impl ProxyHandler {
    pub(crate) fn new(tx: Sender<ProxyData>) -> Self {
        Self {
            tx,
            req_id: Uuid::new_v4(),
        }
    }

}
#[async_trait]
impl HttpHandler for ProxyHandler {
    async fn handle_request(&mut self, _ctx: &HttpContext, req: Request<Body>) -> RequestOrResponse {
        if req.method() == Method::CONNECT {
            return RequestOrResponse::Request(req);
        }
        return match ProxyData::from_req(req).await {
            Ok((mut data, req)) => {
                self.req_id = Uuid::new_v4();
                let uri = req.uri().to_string().clone();
                println!("Request ID: {}, {}", self.req_id, uri);
                data.id = self.req_id.to_string();
                self.tx.send(data).await.unwrap();
                RequestOrResponse::Request(req)
            },
            Err(err) => {
                println!("Error on handling request: {}", err);
                RequestOrResponse::Request(Request::default())
            },
        };
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, res: Response<Body>) -> Response<Body> {
        return match ProxyData::from_res(res).await {
            Ok((mut data, res)) => {
                data.id = self.req_id.to_string();
                println!("Response ID: {}", self.req_id);
                self.tx.send(data).await.unwrap();
                res
            },
            Err(err) => Response::default(),
        };

    }
}
