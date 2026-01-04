use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;


#[derive(Debug, Deserialize,Serialize, FromRow)]
pub struct User{
    pub id:u64,
    pub name:String,
    pub email: String,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub password: String,
    pub role_id : u64,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    updated_at:DateTime<Utc>,
}

#[derive(Debug, Deserialize,Serialize, Validate)]
pub struct UserInsert{
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    pub name:String,
    #[validate(custom(function="crate::utils::validator_utils::validate_email_tld"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password minimal 8 karakter"))]
    pub password: String,
}

#[derive(Deserialize, Validate, Debug,Serialize)]
pub struct UserLogin{
    #[validate(custom(function="crate::utils::validator_utils::validate_email_tld"))]
    pub email : String,
    #[validate(length(min = 5, message = "Password minimal 5 karakter"))]
    pub password: String,
}
