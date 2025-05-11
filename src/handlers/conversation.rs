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

pub async fn get_all(
    Extension(user): Extension<User>,
    Extension(pool): Extension<Pool<Postgres>>
) -> Response {
    let find_result = services::conversation::get_all(
        user.id, 
        &pool
    ).await;
    match find_result {
        Ok(conversations) => return (
                StatusCode::OK,
                Json(conversations)
            ).into_response(),
        Err(err) => return err.into_response()
    }
}

pub async fn delete(
    Path(id): Path<i32>,
    Extension(user): Extension<User>,
    Extension(pool): Extension<Pool<Postgres>>
) -> Response {
    let delete_result = services::conversation::delete(
        id, 
        user.id, 
        &pool
    ).await;
    match delete_result {
        Ok(_) => return (StatusCode::OK).into_response(),
        Err(err) => return err.into_response()
    }
}