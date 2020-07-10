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
