use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize,Serialize)]
pub struct App{
    pub name: String,
    pub ip_address: String,
    pub port: u16,
    pub jwt_secret: String,
    pub api_key: String
}

#[derive(Debug, Deserialize,Serialize)]
pub struct Database{
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String
}

#[derive(Debug, Deserialize,Serialize)]
pub struct Configs{
    pub app: App,
    pub database: Database,
}