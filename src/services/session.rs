use sqlx::{Pool, Postgres};
use uuid::Uuid;
use tracing::error;

use crate::{
    error::AppError,
    modules::user::User
};


pub async fn create(
    user_id: i32,
    pool: &Pool<Postgres>
) -> Result<String, AppError> {
    let session = Uuid::new_v4().to_string();
    let result = sqlx::query(r#"
        INSERT INTO sessions (user_id, session)
        VALUES ($1, $2)
        ON CONFLICT (user_id) DO UPDATE
        SET 
            session = EXCLUDED.session,
            expires_at = CURRENT_TIMESTAMP + INTERVAL '7 days';
    "#)
        .bind(user_id)
        .bind(&session)
        .execute(pool)
        .await;
    match result {
        Ok(_) => return Ok(session),
        Err(e) => match e {
            sqlx::Error::Database(err) => {
                if let Some(err_code) = err.code() {
                    if err_code == "23505" {
                        error!(
                            "The session is found, can not create session for '{}' the uuid is '{}'!", 
                            user_id,
                            session
                        );
                        return Err(AppError::InternalServerError);
                    }
                }
                error!("{:#?}", err);
                return Err(AppError::InternalServerError);
            }
            other => {
                error!("{:#?}", other);
                return Err(AppError::InternalServerError);
            }
        }
    }
}

pub async fn get_user_by_session(
    session: String,
    pool: &Pool<Postgres>
) -> Result<User, AppError> {
    let user = sqlx::query_as::<_, User>(r#"
        SELECT 
            id, 
            name, 
            email, 
            username, 
            password,
            gender,
            to_char(create_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as create_at, 
            to_char(update_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as update_at
        FROM users 
        WHERE
            users.id = (
                SELECT user_id FROM sessions
                WHERE 
                    sessions.session = $1 AND 
                    sessions.expires_at - CURRENT_TIMESTAMP > INTERVAL '0 days'
                LIMIT 1
            );
    "#)
        .bind(&session)
        .fetch_one(pool)
        .await;
    match user {
        Ok(data) => return Ok(data),
        Err(err) => match err {
            sqlx::Error::RowNotFound => return Err(AppError::NotFoundUser),
            other => {
                error!("{:#?}", other);
                return Err(AppError::InternalServerError);
            }
        }
    };
}

pub async fn delete(
    user_id: i32,
    pool: &Pool<Postgres>
) -> Result<(), AppError> {
    let result = sqlx::query(r#"
        DELETE FROM sessions
        WHERE user_id = $1;
    "#)
        .bind(user_id)
        .execute(pool)
        .await;
    match result {
        Ok(_) => return Ok(()),
        Err(err) => match err {
            sqlx::Error::RowNotFound => return Ok(()),
            other => {
                error!("{:#?}", other);
                return Err(AppError::InternalServerError);
            }
        }
    }
}