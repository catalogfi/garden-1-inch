use config::{Config, File};
use serde::{Deserialize, Serialize};

/// Configuration for the watcher service.
/// This struct is used to deserialize the configuration from a TOML file.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WatcherConfig {
    pub core: Core,
    #[serde(rename = "Rpc")]
    pub rpc: Rpc,
    pub contracts: Contracts,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Core {
    pub db_url: String,
    pub polling_interval: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Rpc {
    pub ethereum_rpc: String,
    pub starknet_rpc: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Contracts {
    #[serde(rename = "starknet")]
    pub starknet_contract_address: String,
    #[serde(rename = "ethereum")]
    pub ethereum_contract_address: String,
}

impl WatcherConfig {
    /// Loads watcher configuration from a TOML file.
    ///
    /// # Arguments
    /// * `path` - Path to the TOML configuration file
    ///
    /// # Panics
    /// Will panic if the file cannot be read or if required configuration variables are missing
    pub fn from_toml(path: &str) -> Self {
        let config = Config::builder()
            .add_source(File::with_name(path))
            .build()
            .unwrap_or_else(|_| {
                panic!("Failed to read configuration file at {path}");
            });

        config.try_deserialize().unwrap()
    }

    pub fn from_str(toml_str: &str) -> Self {
        let config = Config::builder()
            .add_source(File::from_str(toml_str, config::FileFormat::Toml))
            .build()
            .expect("Failed to build config from string");

        config.try_deserialize().expect("Deserialization failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_valid_config() {
        let toml_content = r#"
            [core]
            db_url="postgres://king:mangarock@localhost:5432/watcher"
            polling_interval=5000

            [Rpc]
            ethereum_rpc="http://localhost:4040"
            starknet_rpc="http://localhost:5050"

            [contracts]
            starknet="0x"
            ethereum="0x"
            "#;

        let config = WatcherConfig::from_str(toml_content);

        assert_eq!(config.core.polling_interval, 5000);
        assert_eq!(
            config.core.db_url,
            "postgres://king:mangarock@localhost:5432/watcher"
        );
        assert_eq!(config.rpc.ethereum_rpc, "http://localhost:4040");
        assert_eq!(config.rpc.starknet_rpc, "http://localhost:5050");
        assert_eq!(config.contracts.ethereum_contract_address, "0x");
        assert_eq!(config.contracts.starknet_contract_address, "0x");
    }
}
