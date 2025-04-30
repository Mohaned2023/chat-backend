use axum::{middleware, routing::get, Router};
use tracing::info; 

mod middlewares;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(|| async { "Hello, Axum" }))
        .layer(middleware::from_fn(middlewares::logger::log_request));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect(">>> Can NOT create the listener!");
    info!("server running on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect(">>> Axum can NOT serve us!");
}
