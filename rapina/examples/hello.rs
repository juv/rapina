use rapina::prelude::*;

async fn hello(_req: hyper::Request<hyper::body::Incoming>) -> &'static str {
    "Hello, Rapina!"
}

async fn health(_req: hyper::Request<hyper::body::Incoming>) -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let router = Router::new().get("/", hello).get("/health", health);

    Rapina::new().router(router).listen("127.0.0.1:3000").await
}
