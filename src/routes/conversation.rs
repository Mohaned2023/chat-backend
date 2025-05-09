use axum::{
    middleware, 
    routing::{
        delete, 
        get, 
        post
    }, 
    Router
};

use crate::{
    handlers::conversation, 
    middlewares
};

pub fn main() -> Router {
    return Router::new()
        .route("/create/{username}", post(conversation::create))
        .route("/", get(conversation::get_all))
        .route("/delete/{id}", delete(conversation::delete))
        .layer(middleware::from_fn(middlewares::auth::auth_guard));
}