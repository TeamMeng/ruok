use ruok::core::ruok::Ruok;

#[tokio::main]
async fn main() {
    Ruok::new().serve("0.0.0.0:8080").await.unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn add() {
        assert_eq!(1 + 1, 2);
    }
}
