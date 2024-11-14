use crate::idgen;
use crate::proxy::data::ProxyData;
use async_trait::async_trait;
use hudsucker::{
    hyper::{Body, Method, Request, Response},
    HttpContext, HttpHandler, RequestOrResponse,
};
use log::error;
use tauri::async_runtime::Sender;

#[derive(Clone, Debug)]
pub(crate) struct ProxyHandler {
    tx: Sender<ProxyData>,
    req_id: usize,
}

impl ProxyHandler {
    pub(crate) fn new(tx: Sender<ProxyData>) -> Self {
        Self { tx, req_id: 0 }
    }
}
#[async_trait]
impl HttpHandler for ProxyHandler {
    async fn handle_request(
        &mut self,
        ctx: &HttpContext,
        req: Request<Body>,
    ) -> RequestOrResponse {
        if req.method() == Method::CONNECT {
            return RequestOrResponse::Request(req);
        }
        println!("{:?}", ctx);
        match ProxyData::from_req(req).await {
            Ok((mut data, req)) => {
                self.req_id = idgen::gen_id();

                data.id = self.req_id.to_string();
                self.tx.send(data).await.unwrap();
                RequestOrResponse::Request(req)
            }
            Err(err) => {
                error!("Error on handling request: {}", err);
                RequestOrResponse::Request(Request::default())
            }
        }
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, res: Response<Body>) -> Response<Body> {
        match ProxyData::from_res(res).await {
            Ok((mut data, res)) => {
                data.id = self.req_id.to_string();

                self.tx.send(data).await.unwrap();
                res
            }
            Err(err) => {
                error!("Error on handling response: {}", err);
                Response::default()
            }
        }
    }
}
