use rapina::extract::PathParams;
use rapina::prelude::*;

async fn hello(_req: hyper::Request<hyper::body::Incoming>, _params: PathParams) -> &'static str {
    "Hello, Rapina!"
}

async fn health(_req: hyper::Request<hyper::body::Incoming>, _params: PathParams) -> StatusCode {
    StatusCode::OK
}

async fn get_user(_req: hyper::Request<hyper::body::Incoming>, params: PathParams) -> String {
    let id = params.get("id").unwrap();
    format!("ID: {}", id)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let router = Router::new()
        .get("/", hello)
        .get("/health", health)
        .get("/users/:id", get_user);

    Rapina::new().router(router).listen("127.0.0.1:3000").await
}
