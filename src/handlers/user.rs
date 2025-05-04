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
    modules::user::{
        CreateDto, 
        LoginDto
    },
    services,
    utils
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
        Ok(user) => {
            let create_session_result = services::session::create(
                user.id, 
                &pool
            ).await;
            match create_session_result {
                Ok(session) => return (
                        StatusCode::CREATED,
                        utils::create_auth_header(session),
                        Json(user)
                    ).into_response(),
                Err(e) => return e.into_response()
            }
        },
        Err(e) => return e.into_response()
    }
}

pub async fn login(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(login_dto): Json<LoginDto>
) -> Response {
    if let Err(e) = login_dto.validate() {
        return AppError::ValidationError(e.to_string()).into_response();
    }
    let varify_reslt = services::user::login(
        login_dto, 
        &pool
    ).await;
    match varify_reslt {
        Ok(user) => { 
            let create_session_result = services::session::create(
                user.id, 
                &pool
            ).await;
            match create_session_result {
                Ok(session) => return (
                        StatusCode::OK,
                        utils::create_auth_header(session),
                        Json(user)
                    ).into_response(),
                Err(e) => return e.into_response()
            }
        }
        Err(e) => return e.into_response()
    }
}

pub async fn logout() {}

pub async fn refresh() {}

pub async fn get_information() {}

pub async fn update_information() {}

pub async fn update_password() {}

pub async fn delete() {}