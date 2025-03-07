use ruok::core::{request::Request, response_util::ResponseUtil, ruok::Ruok, types::Response};

#[tokio::main]
async fn main() {
    Ruok::new()
        .group("/user")
        .get("/hello", hello)
        .serve("0.0.0.0:8080")
        .await
        .unwrap();
}

async fn hello(_req: Request) -> Response {
    ResponseUtil::string("Hello World", 200)
}

#[cfg(test)]
mod tests {
    #[test]
    fn add() {
        assert_eq!(1 + 1, 2);
    }
}
