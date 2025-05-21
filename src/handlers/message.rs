use axum::{extract::Path, http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use sqlx::{Pool, Postgres};

use crate::{modules::user::User, services};


pub async fn get_all(
    Path(conversation_id): Path<i32>,
    Extension(user): Extension<User>,
    Extension(pool): Extension<Pool<Postgres>>
) -> Response {
    let get_result = services::message::get_all(
        user, 
        conversation_id, 
        &pool
    ).await;
    match get_result {
        Ok(messages) => return (
                StatusCode::OK,
                Json(messages)
            ).into_response(),
        Err(err) => return err.into_response()
    }
}