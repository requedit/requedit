use brotli::Decompressor;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use core::str;
use flate2::read::{DeflateDecoder, GzDecoder};
use hudsucker::hyper::{body::to_bytes, Body, HeaderMap, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Read;
use std::time::SystemTime;
use zstd::stream::read::Decoder as ZstdDecoder;

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
    pub(crate) date: String,
    #[serde(with = "header_map")]
    pub(crate) headers: HeaderMap,
    pub(crate) body: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct HTTPResponse {
    pub(crate) status: String,
    pub(crate) version: String,
    pub(crate) date: String,
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
        let query = parts
            .uri
            .query()
            .map(|q| q.to_string())
            .unwrap_or(String::new());
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
        let content_encoding = headers
            .get("content-encoding")
            .and_then(|ce| ce.to_str().ok());
        let body_text = parse_body(bytes.clone(), content_type, content_encoding).await?;
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
                    date: DateTime::<Utc>::from(SystemTime::now()).to_rfc3339(),
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
        let bytes = match to_bytes(body).await {
            Ok(body_bytes) => body_bytes,
            Err(e) => return Err(RequeditError::from(e)),
        };
        let content_type = headers.get("content-type").and_then(|ct| ct.to_str().ok());
        let content_encoding = headers
            .get("content-encoding")
            .and_then(|ce| ce.to_str().ok());
        let body_text = parse_body(bytes.clone(), content_type, content_encoding).await?;

        Ok((
            Self {
                id: String::default(),
                req: None,
                res: Some(HTTPResponse {
                    status,
                    headers,
                    date: DateTime::<Utc>::from(SystemTime::now()).to_rfc3339(),
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

async fn parse_body(
    body: Bytes,
    content_type: Option<&str>,
    content_encoding: Option<&str>,
) -> Result<String, RequeditError> {
    // 将 `Content-Encoding` 按顺序分割为列表
    let encodings: Vec<&str> = content_encoding
        .unwrap_or("")
        .split(',')
        .map(|e| e.trim())
        .collect();

    // 按编码顺序解压 body
    let decompressed_body = decompress_body(&body, &encodings)?;
    // 获取主要的媒体类型
    let media_type = extract_media_type(content_type);

    match media_type {
        Some("application/json") => {
            let json_value: Value = serde_json::from_slice(&decompressed_body)?;
            Ok(json_value.to_string())
        }
        _ => {
            let text = String::from_utf8_lossy(&decompressed_body).into_owned();
            Ok(text)
        }
    }
}

// 解析 `Content-Type` 头，提取主要类型
fn extract_media_type(content_type: Option<&str>) -> Option<&str> {
    content_type
        .and_then(|ct| ct.split(';').next()) // 取 `application/json; charset=utf-8` 的主要类型
        .map(|media_type| media_type.trim())
}

// 按顺序解压多重编码的 body
fn decompress_body(data: &[u8], encodings: &[&str]) -> Result<Vec<u8>, RequeditError> {
    let mut decoded_data = data.to_vec();

    for encoding in encodings {
        decoded_data = match *encoding {
            "gzip" => {
                let mut decoder = GzDecoder::new(&decoded_data[..]);
                let mut decompressed = Vec::new();
                decoder.read_to_end(&mut decompressed)?;
                decompressed
            }
            "deflate" => {
                let mut decoder = DeflateDecoder::new(&decoded_data[..]);
                let mut decompressed = Vec::new();
                decoder.read_to_end(&mut decompressed)?;
                decompressed
            }
            "br" => {
                let mut decoder = Decompressor::new(&decoded_data[..], 4096);
                let mut decompressed = Vec::new();
                decoder.read_to_end(&mut decompressed)?;
                decompressed
            }
            "zstd" => {
                let mut decoder = ZstdDecoder::new(&decoded_data[..])?;
                let mut decompressed = Vec::new();
                decoder.read_to_end(&mut decompressed)?;
                decompressed
            }
            _ => decoded_data, // 如果编码不支持，则保持原样
        };
    }

    Ok(decoded_data)
}
