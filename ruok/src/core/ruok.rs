use super::{
    response_util::ResponseUtil,
    router::Router,
    types::{Handler, Response},
};
use hyper::{Request, body::Incoming, server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

pub struct Ruok {
    router: Arc<Router>,
}

impl Ruok {
    pub fn new() -> Self {
        Self {
            router: Arc::new(Router::new()),
        }
    }

    pub async fn serve(self, addr: &str) -> Response {
        let addr: SocketAddr = addr.parse()?;

        let listener = TcpListener::bind(addr).await?;
        println!("Server listening on http://{}", addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);

            tokio::spawn(async move {
                let service = service_fn(Ruok::handler_request);

                if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                    println!("Failed to serve connection: {:?}", err);
                }
            });
        }
    }

    async fn handler_request(req: Request<Incoming>) -> Response {
        req.method();
        req.uri().path();
        ResponseUtil::string("404 NOT FOUND", 404)
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

impl Default for Ruok {
    fn default() -> Self {
        Self::new()
    }
}
