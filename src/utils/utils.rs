use config::{Config, File, FileFormat};

use crate::{errors::app_error::AppError, models::config_model::Configs};


pub fn config_load()-> Result<Configs, AppError>{
    let config: Configs = Config::builder()
        .add_source(File::new("config.yaml", FileFormat::Yaml))
        .build()
        ?
        .try_deserialize()
        ?;

    Ok(config)
}