mod web;

use crate::web::{gen_index_html, handle_hello};
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    tracing_subscriber::fmt::init();

    let index_html = gen_index_html();

    let app = Router::new()
        .route("/", get(index_html))
        .route("/api/hello", get(handle_hello))
        .fallback_service(ServeDir::new("frontend/dist"));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::debug!("listening {}", addr);

    if cfg!(debug_assertions) {
        println!("Running at: http://localhost:{}", port);
    };

    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap()
}
