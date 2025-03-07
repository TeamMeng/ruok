use super::types::Handler;
use hyper::Method;
use std::{collections::HashMap, sync::Arc};

pub struct Router {
    routers: HashMap<String, Arc<dyn Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routers: HashMap::new(),
        }
    }

    pub fn add_router(&mut self, path: &str, method: Method, handler: Arc<dyn Handler>) {
        let key = format!("{}_{}", path, method);
        self.routers.insert(key, handler);
    }

    pub fn get(&mut self, path: &str, handler: impl Handler) {
        self.add_router(path, Method::GET, Arc::new(handler));
    }

    pub fn post(&mut self, path: &str, handler: impl Handler) {
        self.add_router(path, Method::POST, Arc::new(handler));
    }

    pub fn put(&mut self, path: &str, handler: impl Handler) {
        self.add_router(path, Method::PUT, Arc::new(handler));
    }

    pub fn delete(&mut self, path: &str, handler: impl Handler) {
        self.add_router(path, Method::DELETE, Arc::new(handler));
    }

    pub fn patch(&mut self, path: &str, handler: impl Handler) {
        self.add_router(path, Method::PATCH, Arc::new(handler));
    }

    pub fn any(&mut self, path: &str, handler: impl Handler) {
        let h = Arc::new(handler);
        self.add_router(path, Method::GET, h.clone());
        self.add_router(path, Method::POST, h.clone());
        self.add_router(path, Method::PUT, h.clone());
        self.add_router(path, Method::DELETE, h.clone());
        self.add_router(path, Method::PATCH, h.clone());
    }

    pub fn match_router(&self, path: &str, method: Method) -> Option<Arc<dyn Handler>> {
        let key = format!("{}_{}", path, method);
        match self.routers.get(&key) {
            Some(handler) => Some(handler.clone()),
            None => None,
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
