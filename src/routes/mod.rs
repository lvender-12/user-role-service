use axum::Router;

use crate::routes::user_router::user_router;



pub mod user_router;

pub fn routes()-> Router{
    Router::new()
        .merge(user_router())
}