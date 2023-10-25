use axum::{extract::Path, routing::get, Router};

async fn greet() -> &'static str {
    "Hello, world!"
}

async fn greet_by_name(name: Path<String>) -> String {
    format!("Hello, {}!", name.to_string())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(greet))
        .route("/:name", get(greet_by_name));

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
