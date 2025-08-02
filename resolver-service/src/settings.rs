use std::collections::HashMap;
use std::env;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub enum ChainType {
    #[serde(rename = "evm")]
    EVM,
    #[serde(rename = "solana")]
    Solana,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub orders_url: String,
    pub poll_interval: u64,
    pub chains: HashMap<String, ChainSettings>,
}

#[derive(Debug, Deserialize)]
pub struct ChainSettings {
    pub chain_type: ChainType,
    pub chain_id: u64,
    pub assets: Vec<String>,
    pub resolver_contract_address: String,
    pub provider: String,
    #[serde(deserialize_with = "deserialize_private_key")]
    pub private_key: String,
}

fn deserialize_private_key<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    
    if value.starts_with("#ENV:") {
        let env_var = &value[5..]; // Remove "#ENV:" prefix
        env::var(env_var).map_err(|_| {
            serde::de::Error::custom(format!("Environment variable '{}' not found", env_var))
        })
    } else {
        Ok(value)
    }
}

impl Settings {
    pub fn from_toml(path: &str) -> Result<Self> {
        let toml_str = std::fs::read_to_string(path)?;
        let settings: Settings = toml::from_str(&toml_str)?;
        Ok(settings)
    }
}
