use axum::{Router, routing::get};

use crate::controllers::user_controller::test_db;

pub fn user_router() -> Router{
    Router::new()
        .route("/user", get(test_db))
}