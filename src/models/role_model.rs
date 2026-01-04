use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct Role {
    pub id: i64,
    pub name: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    updated_at: DateTime<Utc>
}