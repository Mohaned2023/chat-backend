use sqlx::{Pool, Postgres};
use tracing::error;

use crate::{error::AppError, modules::{message::Message, user::User}};


pub async fn get_all(
    user: User,
    conversation_id: i32,
    pool: &Pool<Postgres>
) -> Result<Vec<Message>, AppError> {
    let result = sqlx::query_as::<_, Message>(r#"
        SELECT * FROM messages 
        WHERE
            conversation_id = $1 AND (
                sender_username   = $2 OR
                receiver_username = $2
            );
    "#)
        .bind(conversation_id)
        .bind(&user.username)
        .fetch_all(pool)
        .await;
    match result {
        Ok(messages) => {
            if messages.len() < 1 {
                return Err(AppError::NotFoundData);
            }
            return Ok(messages);
        }
        Err(err) => {
            error!("{:#?}", err);
            return Err(AppError::InternalServerError);
        }
    }
}