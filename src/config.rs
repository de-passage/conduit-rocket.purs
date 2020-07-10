use rocket::config::{Environment, Value};
use std::collections::HashMap;
use std::env;

pub struct Config {
    pub secret: String,
}

impl Config {
    pub fn from_env() -> Result<Config, String> {
        let secret = env::var("SECRET_KEY").or_else(|err| {
            if cfg!(debug_assertions) || cfg!(test) {
                Ok("secret".to_owned())
            } else {
                Err(format!("SECRET_KEY environment variable required: {}", err))
            }
        })?;
        Ok(Config { secret })
    }
}

pub fn configure_rocket() -> Result<rocket::Config, String> {
    let environment = Environment::active().map_err(|err| err.to_string())?;

    let port = if let Ok(port_s) = env::var("PORT") {
        port_s
            .parse::<u16>()
            .map_err(|err| format!("Port parsing failed: {}", err.to_string()))?
    } else {
        8000
    };

    let url = env::var("DATABASE_URL").map_err(|err|
        format!("DATABASE_URL environment variable required, must point to a running Postgresql instance: {}", err.to_string()))?;
    let mut database_config = HashMap::new();
    database_config.insert("url", Value::from(url));

    let mut databases = HashMap::new();
    databases.insert("postgres", Value::from(database_config));

    rocket::Config::build(environment)
        .port(port)
        .extra("databases", databases)
        .finalize()
        .map_err(|err| format!("Rocket configuration failed: {}", err.to_string()))
}
