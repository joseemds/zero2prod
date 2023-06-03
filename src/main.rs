use axum::{extract::Path, routing::get, Router};

async fn greet(Path(username): Path<String>) -> String {
    format!("Hello {}", username)
}

async fn health_check() -> &'static str {
    "Hello world"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/:name", get(greet));
    println!("Hello, world!");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
