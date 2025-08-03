use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::{order_mapper::{OrderAction}, settings::{ChainSettings, ChainType}};

mod evm;



#[async_trait::async_trait]
pub trait Resolver: Send + Sync {
    async fn deploy_src_escrow(&self, order_action: &OrderAction) -> Result<()>;
    async fn deploy_dest_escrow(&self, order_action: &OrderAction) -> Result<()>;
    async fn widthdraw_src_escrow(&self, order_action: &OrderAction) -> Result<()>;
    async fn widthdraw_dest_escrow(&self, order_action: &OrderAction) -> Result<()>;
    async fn arbitrary_calls(&self, order_action: &OrderAction) -> Result<()>;
}

pub async fn create_resolver(chain_settings: &ChainSettings) -> Box<dyn Resolver + Send + Sync> {
    match chain_settings.chain_type {
        ChainType::EVM => Box::new(evm::EvmResolver::new(&chain_settings)),
        _ => panic!("Unsupported chain: {:?}", chain_settings.chain_type),
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomImmutables {
    pub amount: String,
    pub hashlock: String,
    pub maker: String,
    #[serde(rename = "orderHash")]
    pub order_hash: String,
    #[serde(rename = "safetyDeposit")]
    pub safety_deposit: String,
    pub taker: String,
    pub timelocks: String,
    pub token: String,
}