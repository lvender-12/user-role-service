use axum::Json;
use http::StatusCode;

use crate::{db::db::database_pool, errors::app_error::AppError, models::user_model::User};


pub async fn test_db()->Result<(StatusCode, Json<Vec<User>>), AppError>{
    let pool = database_pool().await?;
    let result= sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool).await?;

    Ok((StatusCode::OK, Json(result)))
}