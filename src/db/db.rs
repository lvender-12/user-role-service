use std::time::Duration;

use sqlx::{Error, MySql, Pool, mysql::MySqlPoolOptions};

use crate::{errors::app_error::AppError, utils::utils::config_load};


async fn create_pool(url: String)->Result<Pool<MySql>, Error>{
    MySqlPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(20))
        .idle_timeout(Duration::from_secs(60))
        .connect(&url)
        .await
}

pub async fn database_pool()->Result<Pool<MySql>, AppError>{
    let cfg = config_load()?.database;
    let host = cfg.host;
    let port = cfg.port;
    let user = cfg.user;
    let password = cfg.password;
    let name = cfg.name;

    let database_url = format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, name);

    Ok(create_pool(database_url).await?)
}