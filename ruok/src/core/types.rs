use super::request::Request;
use async_trait::async_trait;
use bytes::Bytes;
use hyper::body::Incoming;
use std::error::Error;

type GenericError = Box<dyn Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

pub type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
pub type Response = Result<hyper::Response<BoxBody>>;
pub type OkRequest = hyper::Request<Incoming>;

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn handler(&self, req: Request) -> Response;
}

#[async_trait]
impl<F: Send + Sync + 'static, Ft> Handler for F
where
    F: Fn(Request) -> Ft,
    Ft: Future<Output = Response> + Send + 'static,
{
    async fn handler(&self, req: Request) -> Response {
        self(req).await
    }
}
