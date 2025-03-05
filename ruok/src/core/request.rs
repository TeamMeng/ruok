use super::types::OkRequest;

pub struct Request {
    pub req: OkRequest,
}

impl Request {
    pub fn new(req: OkRequest) -> Self {
        Self { req }
    }
}
