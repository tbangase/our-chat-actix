pub use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub mysql_host: String,
    pub mysql_port: String,
    pub mysql_database: String,
    pub mysql_user: String,
    pub mysql_password: String,
    pub server_addr: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        // println!("{:?}", cfg.clone().try_into::<Self>().unwrap());
        cfg.try_into()
    }
}
