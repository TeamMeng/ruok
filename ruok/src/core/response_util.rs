use super::types::BoxBody;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
