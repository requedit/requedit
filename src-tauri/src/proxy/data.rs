use core::str;

use bytes::Bytes;
use hudsucker::hyper::{body::to_bytes, Body, HeaderMap, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::RequeditError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ProxyData {
    pub(crate) id: String,
    pub(crate) req: Option<HTTPRequest>,
    pub(crate) res: Option<HTTPResponse>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct HTTPRequest {
    pub(crate) method: String,
    pub(crate) protocol: String,
    pub(crate) host: String,
    pub(crate) path: String,
    pub(crate) query: Option<String>,
    pub(crate) uri: String,
    pub(crate) version: String,
    #[serde(with = "header_map")]
    pub(crate) headers: HeaderMap,
    pub(crate) body: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct HTTPResponse {
    pub(crate) status: String,
    pub(crate) version: String,
    #[serde(with = "header_map")]
    pub(crate) headers: HeaderMap,
    pub(crate) body: String,
}

impl ProxyData {
    pub(crate) async fn from_req(
        req: Request<Body>,
    ) -> Result<(Self, Request<Body>), RequeditError> {
        let (parts, body) = req.into_parts();
        let method = parts.method.to_string();
        let protocol = parts.uri.scheme_str().unwrap_or("http").to_string();
        let host = parts.uri.host().unwrap_or("localhost").to_string();
        let path = parts.uri.path().to_string();
        let query = parts.uri.query().map(|q| q.to_string()).unwrap_or(String::new());
        let headers = parts.headers.clone();
        let version = match parts.version {
            hyper::Version::HTTP_11 => "HTTP/1.1".to_string(),
            hyper::Version::HTTP_09 => "HTTP/0.1".to_string(),
            hyper::Version::HTTP_10 => "HTTP/1.0".to_string(),
            hyper::Version::HTTP_2 => "HTTP/2".to_string(),
            hyper::Version::HTTP_3 => "HTTP/2".to_string(),
            _ => "HTTP/UNKNOWN".to_string(),
        };

        let bytes = match to_bytes(body).await {
            Ok(body_bytes) => body_bytes,
            Err(e) => return Err(RequeditError::from(e)),
        };
        let content_type = headers.get("content-type").and_then(|ct| ct.to_str().ok());
        let body_text = parse_body(bytes.clone(), content_type).await.unwrap_or_else(|e| e.to_string());
        Ok((
            Self {
                id: String::default(),
                req: Some(HTTPRequest {
                    method,
                    protocol,
                    host,
                    path,
                    query: Some(query),
                    uri: parts.uri.to_string(),
                    version,
                    headers: headers,
                    body: body_text,
                }),
                res: None,
            },
            Request::from_parts(parts, Body::from(bytes.clone())),
        ))
    }
    pub(crate) async fn from_res(
        res: Response<Body>,
    ) -> Result<(Self, Response<Body>), RequeditError> {
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
        let content_type = headers.get("content-type").and_then(|ct| ct.to_str().ok());
        let body_text = parse_body(bytes.clone(), content_type).await.unwrap_or_else(|e| e.to_string());

        Ok((
            Self {
                id: String::default(),
                req: None,
                res: Some(HTTPResponse {
                    status,
                    headers,
                    version,
                    body: body_text,
                }),
            },
            Response::from_parts(parts, Body::from(bytes.clone())),
        ))
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

async fn parse_body(bytes: Bytes, content_type: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    // 检查 Content-Type 以确定如何解析
    match content_type {
        Some("application/json") => {
            // JSON 解析
            let json: Value = serde_json::from_slice(&bytes)?;
            Ok(json.to_string())
        }
        Some("text/plain") | Some("text/html") => {
            // 解析为字符串
            let text = str::from_utf8(&bytes)?;
            Ok(text.to_string())
        }
        _ => {
            // 默认转换为十六进制或其他方式进行输出
            Ok(hex::encode(&bytes))
        }
    }
}
