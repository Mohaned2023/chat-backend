use axum::Router;

mod user;
mod conversation;
mod message;

pub fn main() -> Router {
    Router::new()
        .nest("/user", user::main())
        .nest("/conversation", conversation::main())
        .nest("/message", message::main())
}