use argon2::{
    password_hash::{
        rand_core::OsRng, 
        SaltString
    }, 
    Argon2, 
    PasswordHasher
};
use sqlx::{Pool, Postgres};
use tracing::error;

use crate::{error::AppError, modules::user::{CreateDto, User}};


pub async fn create(
    create_dto: CreateDto,
    pool: &Pool<Postgres>
) -> Result<User, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let password = Argon2::default()
        .hash_password(
            &create_dto.password.as_bytes(), 
            &salt
        ).unwrap();
    let result = sqlx::query_as::<_, User>(r#"
        INSERT INTO users (name, username, email, password, gender)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING
            id,
            name,
            username,
            password,
            email,
            gender,
            to_char(create_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as create_at, 
            to_char(update_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as update_at
    "#)
        .bind(&create_dto.name)
        .bind(&create_dto.username)
        .bind(&create_dto.email)
        .bind(&password.to_string())
        .bind(if create_dto.gender.is_some() {create_dto.gender.unwrap()} else {false} )
        .fetch_one(pool)
        .await;
    match result {
        Ok(data) => return Ok(data),
        Err(e) => match e {
            sqlx::Error::Database(db_err) => {
                if let Some(err_code) = db_err.code() {
                    if err_code == "23505" {
                        return Err(AppError::UserFound);
                    }
                }
                error!("{:#?}", db_err);
                return Err(AppError::InternalServerError);
            }
            other => {
                error!("{:#?}", other);
                return Err(AppError::InternalServerError);
            }
        }
    }
}