use super::types::{BoxBody, Response};
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use serde::Serialize;

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

pub struct ResponseUtil;

impl ResponseUtil {
    pub fn string(content: &'static str, code: u16) -> Response {
        Ok(hyper::Response::builder()
            .header("Content-type", "text/plain; charset=UTF-8")
            .status(code)
            .body(full(content))
            .unwrap())
    }

    pub fn html(content: &'static str, code: u16) -> Response {
        Ok(hyper::Response::builder()
            .header("Content-type", "text/html; charset=UTF-8")
            .status(code)
            .body(full(content))
            .unwrap())
    }

    pub fn json<T: Serialize>(data: T, code: u16) -> Response {
        let data = serde_json::to_string(&data).unwrap();
        Ok(hyper::Response::builder()
            .header("Content-type", "application/json; charset=UTF-8")
            .status(code)
            .body(full(data))
            .unwrap())
    }
}
