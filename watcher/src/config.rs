use config::{Config, File};
use serde::{Deserialize, Serialize};
use tracing::warn;

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
    pub ethereum_start_block: u64,
    pub starknet_start_block: u64,
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

        config.try_deserialize().unwrap_or_else(|_| {
            warn!("\nYOU MIGHT HAVE MISSED SOME REQUIRED CONFIGURATION VARIABLES");
            panic!("Failed to deserialize configuration from {path}");
        })
    }

    pub fn from_raw_str(toml_str: &str) -> Self {
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
            db_url = "postgres://king:mangarock@localhost:5432/wallet_db"
            polling_interval = 5000

            [Rpc]
            ethereum_rpc = "https://eth.llamarpc.com"
            starknet_rpc = "https://rpc.starknet.lava.build:443"

            [contracts]
            # starknet = "0x04718f5a0Fc34cC1AF16A1cdee98fFB20C31f5cD61D6Ab07201858f4287c938D"
            starknet = ""
            ethereum = "0x230350B554E468E073B1d44Ce7cD4C6d725dd4a5"
            ethereum_start_block = 23044259
            # starknet_start_block = 0
            starknet_start_block = 1661663

            "#;

        let config = WatcherConfig::from_raw_str(toml_content);
        assert_eq!(
            config.core.db_url,
            "postgres://king:mangarock@localhost:5432/wallet_db"
        );
        assert_eq!(config.core.polling_interval, 5000);
        assert_eq!(config.rpc.ethereum_rpc, "https://eth.llamarpc.com");
        assert_eq!(
            config.rpc.starknet_rpc,
            "https://rpc.starknet.lava.build:443"
        );
        assert_eq!(
            config.contracts.ethereum_contract_address,
            "0x230350B554E468E073B1d44Ce7cD4C6d725dd4a5"
        );
        assert_eq!(config.contracts.ethereum_start_block, 23044259);
        assert_eq!(config.contracts.starknet_contract_address, "");
        assert_eq!(config.contracts.starknet_start_block, 1661663);
    }
}
