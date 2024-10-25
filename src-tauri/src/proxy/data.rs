use bytes::Bytes;
use http_serde::version;
use hudsucker::{
    hyper::{body::to_bytes, Body, HeaderMap, Method, Request, Response, StatusCode, Uri, Version},
    HttpContext, HttpHandler, RequestOrResponse,
};
use serde::{Deserialize, Serialize};

use crate::error::RequeditError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ProxyData {
    index: usize,
    req: Option<HTTPRequest>,
    res: Option<HTTPResponse>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct HTTPRequest {
    method: String,
    protocol: String,
    host: String,
    path: String,
    query: Option<String>,
    version: String,
    #[serde(with = "header_map")]
    headers: HeaderMap,
    body: Option<Bytes>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct HTTPResponse {
    status: String,
    version: String,
    #[serde(with = "header_map")]
    headers: HeaderMap,
    body: Option<Bytes>,
}

impl ProxyData {
    pub(crate) async fn from_req(req: Request<Body>) -> Result<Self, RequeditError> {
        let (parts, body) = req.into_parts();
        let method = parts.method.to_string();
        let protocol = parts.uri.scheme_str().unwrap_or("http").to_string();
        let host = parts.uri.host().unwrap_or("localhost").to_string();
        let path = parts.uri.path().to_string();
        let query = parts.uri.query().map(|q| q.to_string()).unwrap();
        let headers = parts.headers.clone();
        let version = match parts.version {
            hyper::Version::HTTP_11 => "HTTP/1.1".to_string(),
            hyper::Version::HTTP_09 => "HTTP/0.1".to_string(),
            hyper::Version::HTTP_10 => "HTTP/1.0".to_string(),
            hyper::Version::HTTP_2 => "HTTP/2".to_string(),
            hyper::Version::HTTP_3 => "HTTP/2".to_string(),
            _ => "HTTP/UNKNOWN".to_string(),
        };


        let body = match to_bytes(body).await {
            Ok(body_bytes) => body_bytes,
            Err(e) => return Err(RequeditError::from(e)),
        };

        Ok(Self {
            index: 0,
            req: Some(HTTPRequest {
                method,
                protocol,
                host,
                path,
                query: Some(query),
                version,
                headers: headers,
                body: Some(body),
            }),
            res: None,
        })
    }
    async fn from_res(res: Response<Body>) -> Result<Self, RequeditError> {
        let (parts, body) = res.into_parts();
        let bytes = to_bytes(body).await.unwrap();
        let status = parts.status.to_string();
        let headers = parts.headers.clone();
        let version = match parts.version {
            hyper::Version::HTTP_11 => "HTTP/1.1".to_string(),
            hyper::Version::HTTP_09 => "HTTP/0.1".to_string(),
            hyper::Version::HTTP_10 => "HTTP/1.0".to_string(),
            hyper::Version::HTTP_2 => "HTTP/2".to_string(),
            hyper::Version::HTTP_3 => "HTTP/2".to_string(),
            _ => "HTTP/UNKNOWN".to_string(),
        };
        Ok(Self {
            index: 0,
            req: None,
            res: Some(HTTPResponse {
                status,
                headers,
                version,
                body: Some(bytes),
            }),
        })
    }
}

// 序列化和反序列化模块
mod header_map {
    use std::collections::HashMap;

    use super::*;
    use hyper::header::HeaderName;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(headers: &HeaderMap, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let headers: HashMap<String, String> = headers
            .iter()
            .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();
        headers.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HeaderMap, D::Error>
    where
        D: Deserializer<'de>,
    {
        let headers: HashMap<String, String> = HashMap::deserialize(deserializer)?;
        let mut header_map = HeaderMap::new();

        for (key, value) in headers {
            let header_name = key
                .parse::<HeaderName>()
                .map_err(serde::de::Error::custom)?;
            header_map.insert(
                header_name,
                value.parse().map_err(serde::de::Error::custom)?,
            );
        }

        Ok(header_map)
    }
}
