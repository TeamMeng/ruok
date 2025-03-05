use super::{router::Router, types::Handler};
use std::sync::Arc;

#[allow(unused)]
pub struct Ruok {
    router: Arc<Router>,
}

#[allow(unused)]
impl Ruok {
    pub fn new() -> Self {
        Self {
            router: Arc::new(Router::new()),
        }
    }

    pub fn get(mut self, path: &str, handler: impl Handler) -> Self {
        Arc::get_mut(&mut self.router).unwrap().get(path, handler);
        self
    }

    pub fn post(mut self, path: &str, handler: impl Handler) -> Self {
        Arc::get_mut(&mut self.router).unwrap().post(path, handler);
        self
    }

    pub fn delete(mut self, path: &str, handler: impl Handler) -> Self {
        Arc::get_mut(&mut self.router)
            .unwrap()
            .delete(path, handler);
        self
    }

    pub fn put(mut self, path: &str, handler: impl Handler) -> Self {
        Arc::get_mut(&mut self.router).unwrap().put(path, handler);
        self
    }

    pub fn patch(mut self, path: &str, handler: impl Handler) -> Self {
        Arc::get_mut(&mut self.router).unwrap().patch(path, handler);
        self
    }

    pub fn any(mut self, path: &str, handler: impl Handler) -> Self {
        Arc::get_mut(&mut self.router).unwrap().any(path, handler);
        self
    }
}
