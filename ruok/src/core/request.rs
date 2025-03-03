use super::types::OkRequest;

#[allow(unused)]
pub struct Request {
    pub req: OkRequest,
}

#[allow(unused)]
impl Request {
    pub fn new(req: OkRequest) -> Self {
        Self { req }
    }
}
