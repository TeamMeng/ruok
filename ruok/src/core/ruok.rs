use super::{
    request::Request,
    response_util::ResponseUtil,
    router::Router,
    types::{Handler, OkRequest, Response},
};
use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

#[derive(Clone)]
pub enum Status {
    Frame,
    Group(String),
    Router(String),
}

pub struct Ruok {
    prefix: &'static str,
    router: Arc<Router>,
    status: Status,
}

impl Ruok {
    pub fn new() -> Self {
        Self {
            prefix: "",
            router: Arc::new(Router::new()),
            status: Status::Frame,
        }
    }

    pub async fn serve(self, addr: &str) -> Response {
        let addr: SocketAddr = addr.parse()?;

        let listener = TcpListener::bind(addr).await?;
        println!("Server listening on http://{}", addr);

        let owner = Arc::new(self);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);

            let serve = owner.clone();
            tokio::spawn(async move {
                let service = service_fn(move |req| serve.clone().handler_request(req));

                if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                    println!("Failed to serve connection: {:?}", err);
                }
            });
        }
    }

    async fn handler_request(self: Arc<Self>, req: OkRequest) -> Response {
        let path = req.uri().path().to_string();
        let method = req.method().clone();

        let mut handlers = self.router.get_middleware();
        let group_handlers = self.router.match_group_middleware(path.clone());
        handlers = [handlers, group_handlers].concat();

        match self.router.match_router(&path, method) {
            Some(handler) => {
                handlers.push(handler);
            }
            None => handlers.push(Arc::new(not_found)),
        }

        let req = Request::new(req, handlers);
        req.run().await
    }

    pub fn with(mut self, m: impl Handler) -> Self {
        Arc::get_mut(&mut self.router)
            .unwrap()
            .add_middleware(m, self.status.clone());
        self
    }

    pub fn group(mut self, prefix: &'static str) -> Self {
        self.prefix = prefix;
        self.status = Status::Group(prefix.to_string());
        self
    }

    pub fn get(mut self, path: &str, handler: impl Handler) -> Self {
        let path = format!("{}{}", self.prefix, path);
        self.status = Status::Router(path.clone());
        Arc::get_mut(&mut self.router).unwrap().get(&path, handler);
        self
    }

    pub fn post(mut self, path: &str, handler: impl Handler) -> Self {
        let path = format!("{}{}", self.prefix, path);
        self.status = Status::Router(path.clone());
        Arc::get_mut(&mut self.router).unwrap().post(&path, handler);
        self
    }

    pub fn delete(mut self, path: &str, handler: impl Handler) -> Self {
        let path = format!("{}{}", self.prefix, path);
        self.status = Status::Router(path.clone());
        Arc::get_mut(&mut self.router)
            .unwrap()
            .delete(&path, handler);
        self
    }

    pub fn put(mut self, path: &str, handler: impl Handler) -> Self {
        let path = format!("{}{}", self.prefix, path);
        self.status = Status::Router(path.clone());
        Arc::get_mut(&mut self.router).unwrap().put(&path, handler);
        self
    }

    pub fn patch(mut self, path: &str, handler: impl Handler) -> Self {
        let path = format!("{}{}", self.prefix, path);
        self.status = Status::Router(path.clone());
        Arc::get_mut(&mut self.router)
            .unwrap()
            .patch(&path, handler);
        self
    }

    pub fn any(mut self, path: &str, handler: impl Handler) -> Self {
        let path = format!("{}{}", self.prefix, path);
        self.status = Status::Router(path.clone());
        Arc::get_mut(&mut self.router).unwrap().any(&path, handler);
        self
    }
}

impl Default for Ruok {
    fn default() -> Self {
        Self::new()
    }
}

async fn not_found(_req: Request) -> Response {
    ResponseUtil::string("NOT FOUND", 404)
}
