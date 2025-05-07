use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    #[serde(skip)]
    pub password: String,
    pub email: String,
    pub gender: bool,
    pub create_at: String,
    pub update_at: String,
}

#[derive(Validate, Deserialize)]
pub struct CreateDto {
    #[validate(length(min=2, max=100, message="min=2, max=100"))]
    pub name: String,
    #[validate(custom(function="username_validate"))]
    pub username: String,
    #[validate(custom(function="password_validate"))]
    pub password: String,
    #[validate(
        length(min=5, max=100, message="min=5, max=100"),
        email
    )]
    pub email: String,
    pub gender: Option<bool>,
}

#[derive(Validate, Deserialize)]
pub struct LoginDto {
    #[validate(custom(function="username_validate"))]
    pub username: String,
    #[validate(custom(function="password_validate"))]
    pub password: String,
}

#[derive(Validate, Deserialize)]
pub struct UpdateInfoDto {
    #[validate(length(min=2, max=100, message="min=2, max=100"))]
    pub name: Option<String>,
    #[validate(custom(function="username_validate"))]
    pub username: Option<String>,
    #[validate(
        length(min=5, max=100, message="min=5, max=100"),
        email
    )]
    pub email: Option<String>,
    pub gender: Option<bool>,
}

fn username_validate(username: &str) -> Result<(), ValidationError> {
    if username.len() < 3 || username.len() > 50 {
        return Err(ValidationError::new("min=3, max=50"));
    }
    let match_err: Result<(), ValidationError> = Err(ValidationError::new("Username must match ([a-z0-9_]+)"));
    let pattern = Regex::new(r"([a-z0-9_]+)").unwrap();
    let check_result = pattern.captures(username);
    if check_result.is_none() {
        return match_err;
    }
    let get_match_result = check_result.unwrap().get(0);
    if get_match_result.is_none() || get_match_result.unwrap().as_str().len() != username.len() {
        return match_err;
    }
    return Ok(());
}

fn password_validate(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 || password.len() > 512 {
        return Err(ValidationError::new("min=8 && max=512"));
    }
    let lower_letter_pat = Regex::new(r"([a-z])").unwrap();
    let uper_letter_pat = Regex::new(r"([A-Z])").unwrap();
    let numbers_pat = Regex::new(r"([0-9])").unwrap();
    if  !lower_letter_pat.is_match(password) ||
        !uper_letter_pat.is_match(password) ||
        !numbers_pat.is_match(password) {
        return Err(ValidationError::new("password is to wake!!!"));
    }
    Ok(())
}
