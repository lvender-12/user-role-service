use axum::Router;

mod routes;
mod controllers;
mod utils;
mod errors;
mod middlewares;
mod db;
mod models;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes::routes());

    let listener =  tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
