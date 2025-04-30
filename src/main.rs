use axum::{routing::get, Router};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, Axum" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect(">>> Can NOT create the listener!");
    axum::serve(listener, app)
        .await
        .expect(">>> Axum can NOT serve us!");
}
