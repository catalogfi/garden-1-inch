use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;

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
    pub chain_id: String,
    pub assets: Vec<String>,
    pub resolver_contract_address: String,
    pub provider: String,
}


impl Settings {
    pub fn from_toml(path: &str) -> Result<Self> {
        let toml_str = std::fs::read_to_string(path)?;
        let settings: Settings = toml::from_str(&toml_str)?;
        Ok(settings)
    }
}