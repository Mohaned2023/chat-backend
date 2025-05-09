use serde::Serialize;


#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct Conversation {
    pub id: i32,
    pub user1_id: i32,
    pub user2_id: i32,
    pub last_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
