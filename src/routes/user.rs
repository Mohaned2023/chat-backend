use axum::{
    middleware,
    routing::{
        delete, 
        get, 
        patch, 
        post
    },
    Router
};

use crate::{
    handlers::user,
    middlewares
};

pub fn main() -> Router {
    Router::new()
        .route("/logout", get(user::logout))
        .route("/refresh", get(user::refresh))
        .route("/update/info", patch(user::update_information))
        .route("/update/pass", patch(user::update_password))
        .route("/delete", delete(user::delete))
        .route("/info/{username}", get(user::get_information))
        .layer(middleware::from_fn(middlewares::auth::auth_guard))
        .route("/login", post(user::login))
        .route("/register", post(user::register))
}