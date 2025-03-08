use super::{
    response_util::ResponseUtil,
    types::{Handler, OkRequest, Response},
};
use std::sync::Arc;

pub struct Request {
    pub req: OkRequest,
    handlers: Vec<Arc<dyn Handler>>,
    index: usize,
}

impl Request {
    pub fn new(req: OkRequest, handlers: Vec<Arc<dyn Handler>>) -> Self {
        Self {
            req,
            handlers,
            index: 0,
        }
    }

    pub async fn next(self) -> Response {
        self.run().await
    }

    pub async fn run(mut self) -> Response {
        if self.index < self.handlers.len() {
            let handler = self.handlers.get(self.index).unwrap().clone();
            self.index += 1;
            return handler.handler(self).await;
        }
        ResponseUtil::string("Response Error", 500)
    }
}
