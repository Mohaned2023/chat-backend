use serde::Serialize;


#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct Message {
    pub id: i32,
    pub sender_username: String,
    pub receiver_username: String,
    pub conversation_id: i32,
    pub body: String,
    pub delivered: bool,
    pub readed: bool,
    pub created_at: String,
}