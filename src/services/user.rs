use argon2::{
    password_hash::{
        rand_core::OsRng, 
        SaltString
    }, 
    Argon2, 
    PasswordHash, 
    PasswordHasher
};
use sqlx::{Pool, Postgres};
use tracing::error;

use crate::{
    error::AppError, 
    modules::user::{
        CreateDto, 
        LoginDto, 
        UpdateInfoDto, 
        User
    }
};


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

pub async fn login(
    login_dto: LoginDto,
    pool: &Pool<Postgres>
) -> Result<User, AppError> {
    let result = sqlx::query_as::<_, User>(r#"
        SELECT 
            id,
            name,
            username,
            password,
            email,
            gender,
            to_char(create_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as create_at, 
            to_char(update_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as update_at
        FROM users
        WHERE username = $1;
    "#)
        .bind(&login_dto.username)
        .fetch_one(pool)
        .await;
    match result {
        Ok(user) => {
            if let Ok(parsed_hash) = PasswordHash::new(&user.password) {
                let verify_result = parsed_hash
                    .verify_password(
                        &[&Argon2::default()], 
                        login_dto.password
                    );
                if verify_result.is_ok() {
                    return Ok(user);
                }
            }
            return Err(AppError::Unauthorized);
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => return Err(AppError::Unauthorized),
            other => {
                error!("{:#?}", other);
                return Err(AppError::InternalServerError);
            }
        }
    }
}

pub async fn find(
    username: String,
    pool: &Pool<Postgres>
) -> Result<User, AppError> {
    let result = sqlx::query_as::<_, User>(r#"
        SELECT 
            id,
            name,
            username,
            password,
            email,
            gender,
            to_char(create_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as create_at, 
            to_char(update_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as update_at
        FROM users
        WHERE username = $1;
    "#)
        .bind(&username)
        .fetch_one(pool)
        .await;
    match result {
        Ok(user) => return Ok(user),
        Err(err) => match err {
            sqlx::Error::RowNotFound => return Err(AppError::NotFoundUser),
            other => {
                error!("{:#?}", other);
                return Err(AppError::InternalServerError);
            }
        }
    }
}

pub async fn update_information(
    user: User,
    update_info_dto: UpdateInfoDto,
    pool: &Pool<Postgres>
) -> Result<User, AppError> {
    if  update_info_dto.name.is_none()     &&
        update_info_dto.username.is_none() &&
        update_info_dto.email.is_none()    &&
        update_info_dto.gender.is_none() {
        return Err(AppError::BadRequest);
    }
    let result = sqlx::query_as::<_, User>(r#"
        UPDATE users
        SET
            name     = $1,
            username = $2,
            email    = $3,
            gender   = $4
        WHERE 
            id = $5
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
        .bind( & if update_info_dto.name.is_some() { update_info_dto.name.unwrap() } else { user.name })
        .bind( & if update_info_dto.username.is_some() { update_info_dto.username.unwrap() } else { user.username })
        .bind( & if update_info_dto.email.is_some() { update_info_dto.email.unwrap() } else { user.email })
        .bind( & if update_info_dto.gender.is_some() { update_info_dto.gender.unwrap() } else { user.gender })
        .bind( user.id )
        .fetch_one(pool)
        .await;
    match result {
        Ok(data) => return Ok(data),
        Err(err) => match err {
            sqlx::Error::Database(e) => {
                if let Some(code) = e.code() {
                    if code == "23505" {
                        return Err(AppError::UserFound);
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
