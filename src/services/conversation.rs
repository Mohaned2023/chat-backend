use sqlx::{Pool, Postgres};
use tracing::error;

use crate::{error::AppError, modules::{conversation::Conversation, user::User}};



pub async fn create(
    username: String,
    user: User,
    pool: &Pool<Postgres>
) -> Result<Conversation, AppError> {
    if username == user.username {
        return Err(AppError::BadRequest);
    }
    let result = sqlx::query_as::<_, Conversation>(r#"
        INSERT INTO conversations (user1_id, user2_id)
        VALUES (
            $1, 
            (
                SELECT 
                    id as user2_id
                FROM users
                WHERE 
                    username = $2
            )
        )
        RETURNING
            id,
            user1_id,
            user2_id,
            last_message,
            to_char(created_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at, 
            to_char(updated_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as updated_at;
    "#)
        .bind(user.id)
        .bind(&username)
        .fetch_one(pool)
        .await;
    match result {
        Ok(conversation) => return Ok(conversation),
        Err(err) => match err {
            sqlx::Error::Database(e) => {
                if let Some(err_code) = e.code() {
                    if err_code == "23502" {
                        return Err(AppError::NotFoundUser);
                    }
                }
                error!("{:#?}", e);
                return Err(AppError::InternalServerError);
            }
            other => {
                error!("{:#?}", other);
                return Err(AppError::InternalServerError);
            }
        }
    }
}