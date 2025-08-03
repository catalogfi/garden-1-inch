use config::{Config, ConfigError, File};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub db_url: String,
    pub port: u16,
}

impl Settings {
    pub fn from_toml(path: &str) -> Self {
        match Self::try_from_toml(path) {
            Ok(settings) => settings,
            Err(e) => {
                tracing::error!("Failed to load settings from {}: {}", path, e);
                panic!("Missing required configuration variables : {}", e);
            }
        }
    }

    fn try_from_toml(path: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(path))
            .build()?;
        config.try_deserialize()
    }
}
