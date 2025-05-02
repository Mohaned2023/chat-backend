use axum::{
    http::StatusCode, 
    response::{
        IntoResponse, 
        Response
    }, 
    Extension, 
    Json
};
use sqlx::{Pool, Postgres};
use validator::Validate;

use crate::{
    error::AppError, 
    modules::user::CreateDto, services
};


pub async fn register(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(create_dto): Json<CreateDto>
) -> Response {
    if let Err(err) = create_dto.validate() {
        return AppError::ValidationError(err.to_string()).into_response();
    }
    let create_result = services::user::create(
        create_dto, 
        &pool
    ).await;
    match create_result {
        Ok(user) => return (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => return e.into_response()
    }
}

pub async fn login() {}

pub async fn logout() {}

pub async fn refresh() {}

pub async fn get_information() {}

pub async fn update_information() {}

pub async fn update_password() {}

pub async fn delete() {}