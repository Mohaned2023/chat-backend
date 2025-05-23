use axum::{
    extract::Path, 
    http::StatusCode, 
    response::{
        IntoResponse, 
        Response
    }, 
    Extension, 
    Json
};
use serde_json::json;
use sqlx::{Pool, Postgres};
use validator::Validate;

use crate::{
    error::AppError, 
    modules::user::{
        CreateDto, 
        LoginDto,
        UpdateInfoDto, 
        UpdatePassDto, 
        User
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
                        utils::create_auth_header(session.clone()),
                        Json(json!({
                            "session_id": session,
                            "user": user
                        }))
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
                        utils::create_auth_header(session.clone()),
                        Json(json!({
                            "session_id": session,
                            "user": user
                        }))
                    ).into_response(),
                Err(e) => return e.into_response()
            }
        }
        Err(e) => return e.into_response()
    }
}

pub async fn logout(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(user): Extension<User>
) -> Response {
    let delete_session_result = services::session::delete(
        user.id, 
        &pool
    ).await;
    match delete_session_result {
        Ok(_) => return (
                StatusCode::OK, 
                utils::empty_auth_header()
            ).into_response(),
        Err(err) => return err.into_response()
    }
}

pub async fn refresh(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(user): Extension<User>
) -> Response {
    let create_session_result = services::session::create(
        user.id, 
        &pool
    ).await;
    match create_session_result {
        Ok(session) => return (
                StatusCode::OK,
                utils::create_auth_header(session)
            ).into_response(),
        Err(err) => return err.into_response()
    }
}

pub async fn get_information(
    Path(username): Path<String>,
    Extension(user): Extension<User>,
    Extension(pool): Extension<Pool<Postgres>>
) -> Response {
    if username == user.username {
        return (StatusCode::OK, Json(user)).into_response();
    }
    let find_result = services::user::find(
        username, 
        &pool
    ).await;
    match find_result {
        Ok(data) => return (
                StatusCode::OK,
                Json(json!({
                    "id": data.id,
                    "name": data.name,
                    "username": data.username,
                    "gender": data.gender,
                    "email": data.email
                }))
            ).into_response(),
        Err(err) => return err.into_response()
    }
}

pub async fn update_information(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(user): Extension<User>,
    Json(update_info_dto): Json<UpdateInfoDto>
) -> Response {
    if let Err(err) = update_info_dto.validate() {
        return AppError::ValidationError(err.to_string()).into_response();
    }
    let update_result = services::user::update_information(
        user, 
        update_info_dto, 
        &pool
    ).await;
    match update_result {
        Ok(data) => return (
                StatusCode::OK,
                Json(data)
            ).into_response(),
        Err(e) => return e.into_response()
    }
}

pub async fn update_password(
    Extension(user): Extension<User>,
    Extension(pool): Extension<Pool<Postgres>>,
    Json(update_pass_dto): Json<UpdatePassDto>
) -> Response {
    if let Err(err) = update_pass_dto.validate() {
        return AppError::ValidationError(err.to_string()).into_response();
    }
    let update_result = services::user::update_password(
        user, 
        update_pass_dto, 
        &pool
    ).await;
    match update_result {
        Ok(_) => return (StatusCode::OK).into_response(),
        Err(err) => return err.into_response()
    }
}

pub async fn delete(
    Extension(user): Extension<User>,
    Extension(pool): Extension<Pool<Postgres>>
) -> Response {
    let delete_result = services::user::delete(
        user, 
        &pool
    ).await;
    match delete_result {
        Ok(_) => return (
                StatusCode::OK,
                utils::empty_auth_header()
            ).into_response(),
        Err(err) => return err.into_response()
    }
}