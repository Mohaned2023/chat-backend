use axum::Router;

mod user;
mod conversation;

pub fn main() -> Router {
    Router::new()
        .nest("/user", user::main())
        .nest("/conversation", conversation::main())
}