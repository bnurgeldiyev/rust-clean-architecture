use std::time::{Duration, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use sqlx::types::chrono::{Utc};
use chrono::prelude::DateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserCreateRequest {
    pub username: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct UserGet {
    pub id: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct UserGetPassword {
    pub password: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(sqlx::FromRow)]
pub struct UserFromDb {
    pub id: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub create_ts: i64,
    pub update_ts: i64,
}

#[derive(sqlx::FromRow)]
pub struct UserEmpty {
    pub id: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGetResponse {
    pub id: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub create_ts: DateTime<Utc>,
    pub update_ts: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdateRequest {
    pub id: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdateResponse {
    pub id: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub create_ts: DateTime<Utc>,
    pub update_ts: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserChangePasswordRequest {
    pub id: i32,
    pub old_password: String,
    pub new_password: String,
}

pub fn convert_unix_to_date(date: i64) -> DateTime<Utc> {
    let d = UNIX_EPOCH + Duration::from_secs(date as u64);
    let datetime = DateTime::<Utc>::from(d);
    datetime
}

// verifying

pub fn verify_user_auth_request(req: UserAuthRequest) -> Result<(), String> {
    if req.username.len() < 5 {
        return Err("invalid username".to_string());
    }

    if req.password.len() < 5 {
        return Err("invalid password".to_string());
    }

    Ok(())
}

pub fn verify_user_create_request(req: UserCreateRequest) -> Result<(), String> {
    if req.username.len() < 5 {
        return Err("invalid username".to_string());
    }

    if req.password.len() < 5 {
        return Err("invalid password".to_string());
    }

    if req.firstname.len() == 0 {
        return Err("firstname is empty".to_string());
    }

    if req.lastname.len() == 0 {
        return Err("lastname is empty".to_string());
    }

    Ok(())
}
