use axum::{middleware, routing::get, Router};

use crate::{handlers::message, middlewares};


pub fn main() -> Router {
    Router::new()
        .route("/{id}", get(message::get_all))
        .layer(middleware::from_fn(middlewares::auth::auth_guard))
}