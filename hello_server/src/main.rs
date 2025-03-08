use ruok::core::{request::Request, response_util::ResponseUtil, ruok::Ruok, types::Response};

#[tokio::main]
async fn main() {
    Ruok::new()
        .with(log)
        .group("/user")
        .with(auth)
        .get("/hello", hello)
        .post("/login", login)
        .serve("0.0.0.0:8080")
        .await
        .unwrap();
}

async fn hello(_req: Request) -> Response {
    println!("hello");
    ResponseUtil::string("Hello World", 200)
}

async fn log(req: Request) -> Response {
    println!("log before");
    let rsp = req.next().await;
    println!("log after");
    rsp
}

async fn auth(req: Request) -> Response {
    println!("auth before");
    let rsp = req.next().await;
    println!("auth after");
    rsp
}

async fn login(_req: Request) -> Response {
    ResponseUtil::string("Login", 200)
}

#[cfg(test)]
mod tests {
    #[test]
    fn add() {
        assert_eq!(1 + 1, 2);
    }
}
