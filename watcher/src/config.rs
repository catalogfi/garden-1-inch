use config::{Config, File};
use serde::{Deserialize, Serialize};
use tracing::warn;

/// Configuration for the watcher service.
/// This struct is used to deserialize the configuration from a TOML file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatcherConfig {
    pub core: CoreConfig,
    pub chains: ChainsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConfig {
    pub db_url: String,
    pub polling_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainsConfig {
    pub evm: Vec<EvmChainConfig>,
    pub starknet: Vec<StarknetChainConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvmChainConfig {
    pub name: String,
    pub chain_id: u64,
    pub rpc_url: String,
    pub contract_address: String,
    pub start_block: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarknetChainConfig {
    pub name: String,
    pub chain_id: String,
    pub rpc_url: String,
    pub contract_address: String,
    pub start_block: u64,
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

            [[chains.evm]]
            name = "ethereum"
            chain_id = 1
            rpc_url = "https://eth.llamarpc.com"
            contract_address = "0x7E030bC01EBFca5c1088f7f281D0c73bb8C50D54"
            start_block = 29182503

            [[chains.evm]]
            name = "base"
            chain_id = 8453
            rpc_url = "https://base.llamarpc.com"
            contract_address = "0x1234567890123456789012345678901234567890"
            start_block = 1000000

            [[chains.evm]]
            name = "monad"
            chain_id = 60808
            rpc_url = "https://testnet1.monad.xyz"
            contract_address = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd"
            start_block = 100

            [[chains.starknet]]
            name = "starknet-mainnet"
            chain_id = "SN_MAIN"
            rpc_url = "https://rpc.starknet.lava.build:443"
            contract_address = "0x04718f5a0Fc34cC1AF16A1cdee98fFB20C31f5cD61D6Ab07201858f4287c938D"
            start_block = 1661663
              
            [[chains.starknet]]
            name = "starknet-sepolia"
            chain_id = "SN_SEPOLIA"
            rpc_url = "https://starknet-sepolia.public.blastapi.io/rpc/v0_7"
            contract_address = "0x1234567890abcdef1234567890abcdef12345678"
            start_block = 100000
            "#;

        let config = WatcherConfig::from_raw_str(toml_content);
        assert_eq!(
            config.core.db_url,
            "postgres://king:mangarock@localhost:5432/wallet_db"
        );
        assert_eq!(config.core.polling_interval, 5000);
        assert_eq!(config.chains.evm.len(), 3);
        assert_eq!(config.chains.starknet.len(), 2);
    }
}
