use axum::{
    extract::Request,
    middleware::Next,
    response::{
        IntoResponse, 
        Response
    }, Extension
};
use axum_extra::extract::CookieJar;
use sqlx::{Pool, Postgres};

use crate::{
    error::AppError, 
    services
};


pub async fn auth_guard(
    Extension(pool): Extension<Pool<Postgres>>,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Response{
    let get_session_result = jar.get("session");
    match get_session_result {
        Some(session_id) => {
            let get_user_result = services::session::get_user_by_session(
                session_id.value().to_string(), 
                &pool
            ).await;
            match get_user_result {
                Ok(user) => {
                    req.extensions_mut().insert(user);
                    return next.run(req).await;
                }
                Err(e) => {
                    if e == AppError::NotFoundUser {
                        return AppError::Unauthorized.into_response();
                    }
                    return e.into_response();
                }
            }
        }
        None =>  return AppError::Unauthorized.into_response()
    }
}