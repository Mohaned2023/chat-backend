use axum::{middleware, Extension, Router};
use tracing::info;
use dotenvy::dotenv;

mod middlewares;
mod db;
mod routes;
mod handlers;
mod modules;
mod error;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let db_conn = db::create_db_connection().await;
    info!("Database connection created.");
    let app = Router::new()
        .nest("/api/v1", routes::main())
        .layer(middleware::from_fn(middlewares::logger::log_request))
        .layer(Extension(db_conn));
    let port = std::env::var("CHAT_APP_PORT")
        .expect(">>> CHAT_APP_PORT NOT found!");
    let listener = tokio::net::TcpListener::bind(
        format!("127.0.0.1:{}", port)
    )
        .await
        .expect(">>> Can NOT create the listener!");
    info!("server running on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect(">>> Axum can NOT serve us!");
}
