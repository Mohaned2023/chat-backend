use sqlx::{Pool, Postgres};
use uuid::Uuid;
use tracing::error;

use crate::error::AppError;


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