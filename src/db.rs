use sqlx::{
    postgres::PgPoolOptions, 
    Pool, 
    Postgres
};

pub async fn create_db_connection() -> Pool<Postgres> {
    let db_url = std::env::var("CHAT_DATABASE_URL")
        .expect(">>> CHAT_DATABASE_URL NOT found!");
    return PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect(">>> Can NOT connect to database!");
}