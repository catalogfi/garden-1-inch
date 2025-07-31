use anyhow::Result;
use crate::{order_mapper::{OrderAction}, settings::{ChainSettings, ChainType}};

mod evm;

#[async_trait::async_trait]
pub trait Resolver: Send + Sync {
    async fn deploy_escrow(&self, order_action: &OrderAction) -> Result<()>;
    async fn release_funds(&self, order_action: &OrderAction) -> Result<()>;
    async fn refund_funds(&self, order_action: &OrderAction) -> Result<()>;
}

pub async fn create_resolver(chain_settings: &ChainSettings) -> Box<dyn Resolver + Send + Sync> {
    match chain_settings.chain_type {
        ChainType::EVM => Box::new(evm::EvmResolver::new(&chain_settings)),
        _ => panic!("Unsupported chain: {:?}", chain_settings.chain_type),
    }
}