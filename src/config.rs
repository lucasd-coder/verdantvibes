use std::env;

use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}
#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
}

impl AppConfig {
    pub fn new() -> Self {
        let env = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
        let config_file = format!("config/{}.yaml", env);

        // Load configuration from the appropriate file
        let settings = Config::builder()
            .add_source(config::File::with_name(&config_file))
            .build()
            .unwrap();

        // Deserialize into AppConfig
        let config: AppConfig = settings.try_deserialize::<AppConfig>().unwrap();
        config
    }
}
