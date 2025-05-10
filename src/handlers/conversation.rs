use axum::{extract::Path, http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use sqlx::{Pool, Postgres};

use crate::{modules::user::User, services};


pub async fn create(
    Path(username): Path<String>,
    Extension(user): Extension<User>,
    Extension(pool): Extension<Pool<Postgres>>
) -> Response {
    let create_result = services::conversation::create(
        username, 
        user, 
        &pool
    ).await;
    match create_result {
        Ok(conversation) => return (
                StatusCode::CREATED,
                Json(conversation)
            ).into_response(),
        Err(err) => return err.into_response()
    }
}

pub async fn get_all() {}

pub async fn delete() {}