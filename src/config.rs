use config::{Config, ConfigError};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct MongoConfig {
    #[serde(default = "default_mongo_uri")]
    pub uri: String,
    #[serde(default = "default_mongo_database")]
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct HoyolabConfig {
    pub region: String,
    pub uid: String,
    pub cookie: String,
    pub user_agent: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub mongodb: MongoConfig,
    pub hoyolab: HoyolabConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "development".into());
        let config_dir = PathBuf::from("config");
        
        let config = Config::builder()
            .add_source(config::File::from(config_dir.join("default")).required(false))
            .add_source(config::File::from(config_dir.join(&env)).required(true))
            .build()?;

        let settings: Settings = config.try_deserialize().map_err(|e| {
            ConfigError::Message(format!(
                "Failed to deserialize config: {}. Make sure Hoyolab credentials are provided in the config file",
                e
            ))
        })?;

        // Validate required fields
        if settings.hoyolab.region.is_empty() {
            return Err(ConfigError::Message("Hoyolab region is required".into()));
        }
        if settings.hoyolab.uid.is_empty() {
            return Err(ConfigError::Message("Hoyolab UID is required".into()));
        }
        if settings.hoyolab.cookie.is_empty() {
            return Err(ConfigError::Message("Hoyolab cookie is required".into()));
        }

        Ok(settings)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

impl Default for MongoConfig {
    fn default() -> Self {
        Self {
            uri: default_mongo_uri(),
            database: default_mongo_database(),
        }
    }
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    7878
}

fn default_mongo_uri() -> String {
    "mongodb://localhost:27017".to_string()
}

fn default_mongo_database() -> String {
    "starrail-api".to_string()
} 