use super::{ruok::Status, types::Handler};
use hyper::Method;
use std::{collections::HashMap, sync::Arc};

pub struct Router {
    routers: HashMap<String, Arc<dyn Handler>>,
    middle_handlers: Vec<Arc<dyn Handler>>,
    group_middle_handlers: HashMap<String, Vec<Arc<dyn Handler>>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routers: HashMap::new(),
            middle_handlers: Vec::new(),
            group_middle_handlers: HashMap::new(),
        }
    }

    pub fn add_middleware(&mut self, m: impl Handler, status: Status) {
        match status {
            Status::Frame => self.middle_handlers.push(Arc::new(m)),
            Status::Group(prefix) => match self.group_middle_handlers.get_mut(prefix.as_str()) {
                Some(v) => v.push(Arc::new(m)),
                None => {
                    self.group_middle_handlers.insert(prefix, vec![Arc::new(m)]);
                }
            },
            Status::Router(path) => panic!("Can't add path: {} to middleware", path),
        }
    }

    pub fn get_middleware(&self) -> Vec<Arc<dyn Handler>> {
        self.middle_handlers.clone()
    }

    pub fn match_group_middleware(&self, path: String) -> Vec<Arc<dyn Handler>> {
        let keys = self.group_middle_handlers.keys();
        for k in keys {
            let prefix = format!("{}{}", k, "/");
            if path.contains(&prefix) {
                let vec = self.group_middle_handlers.get(k).unwrap().clone();
                return vec;
            }
        }
        vec![]
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
