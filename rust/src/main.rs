use std::env;

use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = env::var("PORT")
        .map(|s| s.parse::<u16>().ok())
        .ok()
        .flatten()
        .unwrap_or(3000);

    let app = Router::new().route(
        "/",
        get(|| async { Html("Hello, World from rust plugin template!") }),
    );

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
