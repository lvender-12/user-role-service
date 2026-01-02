use axum::{Router, routing::get};

use crate::controllers::user_controller::hallo;

pub fn user_router() -> Router{
    Router::new()
        .route("/user", get(hallo))
}