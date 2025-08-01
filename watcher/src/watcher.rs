use crate::{
    chains::{ethereum::EthereumChain, starknet::StarknetChain, traits::Chain},
    orderbook::provider::OrderbookProvider,
    types::ChainType,
};
use std::sync::Arc;
use tracing::info;

pub enum ChainWatcher {
    Ethereum(EthereumChain),
    Starknet(StarknetChain),
}

pub struct Watcher {
    chain: ChainWatcher,
}

impl Watcher {
    pub async fn new(
        rpc_url: String,
        contract_address: String,
        chain_type: ChainType,
        db: Arc<OrderbookProvider>,
        start_block: u64,
    ) -> anyhow::Result<Self> {
        let chain = match chain_type {
            ChainType::Ethereum => ChainWatcher::Ethereum(
                EthereumChain::new(rpc_url, contract_address, db, start_block).await?,
            ),
            ChainType::Starknet => ChainWatcher::Starknet(
                StarknetChain::new(rpc_url, contract_address, db, start_block).await?,
            ),
        };

        Ok(Self { chain })
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        info!("Starting watcher service");
        match &mut self.chain {
            ChainWatcher::Ethereum(chain) => chain.start().await,
            ChainWatcher::Starknet(chain) => chain.start().await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chains::ethereum::*;
    use crate::chains::starknet::StarknetChain;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_ethereum_chain() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();

        let db_url = "postgres://king:mangarock@localhost:5432/wallet_db";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;
        let db = Arc::new(OrderbookProvider::new(pool));

        let mut chain = EthereumChain::new(
            "https://eth.llamarpc.com".to_string(),
            "0x230350B554E468E073B1d44Ce7cD4C6d725dd4a5".to_string(),
            db,
            23041963,
        )
        .await?;

        chain.start().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_starknet_chain() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();

        let db_url = "postgres://king:mangarock@localhost:5432/wallet_db";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;
        let db = Arc::new(OrderbookProvider::new(pool));

        let mut chain = StarknetChain::new(
            "https://rpc.starknet.lava.build:443".to_string(),
            "0x04718f5a0Fc34cC1AF16A1cdee98fFB20C31f5cD61D6Ab07201858f4287c938D".to_string(),
            db,
            1661663,
        )
        .await?;

        chain.start().await?;
        Ok(())
    }
}
