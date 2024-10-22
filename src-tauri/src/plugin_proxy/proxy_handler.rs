use async_trait::async_trait;
use bytes::Bytes;
use hudsucker::{
    hyper::{body::to_bytes, Body, HeaderMap, Method, Request, Response, StatusCode, Uri, Version},
    HttpContext, HttpHandler, RequestOrResponse,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::async_runtime::Sender;

#[derive(Clone, Debug)]
pub(crate) struct ProxyHandler {
    tx: Sender<ProxyHandler>, // TODO: 
    req: Option<ProxiedRequest>,
    res: Option<ProxiedResponse>,
}

impl ProxyHandler {
    pub(crate) fn new(tx: Sender<ProxyHandler>) -> Self {
        Self {
            tx,
            req: None,
            res: None,
        }
    }

    pub(crate) fn to_parts(self) -> (Option<ProxiedRequest>, Option<ProxiedResponse>) {
        (self.req, self.res)
    }

    pub(crate) fn set_req(&mut self, req: ProxiedRequest) -> Self {
        Self {
            tx: self.clone().tx,
            req: Some(req),
            res: None,
        }
    }

    pub(crate) fn set_res(&mut self, res: ProxiedResponse) -> Self {
        Self {
            tx: self.clone().tx,
            req: self.clone().req,
            res: Some(res),
        }
    }

    pub(crate) async fn send_output(self) {
        if let Err(e) = self.tx.send(self.clone()).await {
            eprintln!("Error on sending Response to main thread: {}", e);
        }
    }
}
#[async_trait]
impl HttpHandler for ProxyHandler {
    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        req: Request<Body>,
    ) -> RequestOrResponse {
        let (parts, body) = req.into_parts();
        let bytes = to_bytes(body).await.unwrap();

        let output_request = ProxiedRequest::new(
            parts.method.clone(),
            parts.uri.clone(),
            parts.version,
            parts.headers.clone(),
            bytes.clone(),
            chrono::Local::now()
                .timestamp_nanos_opt()
                .unwrap_or_default(),
        );
        self.set_req(output_request).send_output().await;

        RequestOrResponse::Request(Request::from_parts(parts, Body::from(bytes.clone())))
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, res: Response<Body>) -> Response<Body> {
        let (parts, body) = res.into_parts();
        let bytes = to_bytes(body).await.unwrap();
        let output_response = ProxiedResponse::new(
            parts.status.clone(),
            parts.version,
            parts.headers.clone(),
            bytes.clone(),
            chrono::Local::now()
                .timestamp_nanos_opt()
                .unwrap_or_default(),
        );

        self.set_res(output_response).send_output().await;
        // 创建新的 Body
        let new_body = Body::from(bytes.clone());
        let new_res = Response::from_parts(parts, new_body);
        new_res
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct ProxiedRequest {
    #[serde(with = "http_serde::method")]
    method: Method,
    #[serde(with = "http_serde::uri")]
    uri: Uri,
    #[serde(with = "http_serde::version")]
    version: Version,
    #[serde(with = "http_serde::header_map")]
    headers: HeaderMap,
    body: Bytes,
    time: i64,
}

impl ProxiedRequest {
    pub(crate) fn new(
        method: Method,
        uri: Uri,
        version: Version,
        headers: HeaderMap,
        body: Bytes,
        time: i64,
    ) -> Self {
        Self {
            method,
            uri,
            version,
            headers,
            body,
            time,
        }
    }

    // pub fn method(&self) -> &Method {
    //     &self.method
    // }

    // pub fn uri(&self) -> &Uri {
    //     &self.uri
    // }

    // pub fn version(&self) -> &Version {
    //     &self.version
    // }

    // pub fn headers(&self) -> &HeaderMap {
    //     &self.headers
    // }

    // pub fn body(&self) -> &Bytes {
    //     &self.body
    // }

    // pub fn time(&self) -> i64 {
    //     self.time
    // }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct ProxiedResponse {
    #[serde(with = "http_serde::status_code")]
    status: StatusCode,
    #[serde(with = "http_serde::version")]
    version: Version,
    #[serde(with = "http_serde::header_map")]
    headers: HeaderMap,
    body: Bytes,
    time: i64,
}

impl ProxiedResponse {
    pub(crate) fn new(
        status: StatusCode,
        version: Version,
        headers: HeaderMap,
        body: Bytes,
        time: i64,
    ) -> Self {
        Self {
            status,
            version,
            headers,
            body,
            time,
        }
    }

    // pub fn status(&self) -> &StatusCode {
    //     &self.status
    // }

    // pub fn version(&self) -> &Version {
    //     &self.version
    // }

    // pub fn headers(&self) -> &HeaderMap {
    //     &self.headers
    // }

    // pub fn body(&self) -> &Bytes {
    //     &self.body
    // }

    // pub fn time(&self) -> i64 {
    //     self.time
    // }
}

trait ToString {
    fn to_string(&self) -> String;
}

trait ToHashString {
    fn to_hash_string(&self) -> HashMap<String, String>;
}

impl ToHashString for HeaderMap {
    fn to_hash_string(&self) -> HashMap<String, String> {
        let mut headers: HashMap<String, String> = HashMap::new();

        for (k, v) in self.iter() {
            headers
                .insert(k.as_str().to_string(), v.to_str().unwrap().to_string())
                .unwrap_or("NO header".to_string());
        }
        headers
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        match *self {
            Version::HTTP_09 => "HTTP_09".to_string(),
            Version::HTTP_10 => "HTTP_10".to_string(),
            Version::HTTP_11 => "HTTP_11".to_string(),
            Version::HTTP_2 => "HTTP_2".to_string(),
            Version::HTTP_3 => "HTTP_3".to_string(),
            _ => "__NonExhaustive".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct RequestInfo(pub Option<ProxiedRequest>, pub Option<ProxiedResponse>);
