use rapina::prelude::*;

#[get("/")]
async fn hello() -> &'static str {
    "Hello, Rapina!"
}

#[get("/health")]
async fn health() -> StatusCode {
    StatusCode::OK
}

#[get("/users/:id")]
async fn get_user(id: Path<u64>) -> String {
    format!("ID: {}", id.into_inner())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let router = Router::new()
        .get("/", hello)
        .get("/health", health)
        .get("/users/:id", get_user);

    Rapina::new().router(router).listen("127.0.0.1:3000").await
}
