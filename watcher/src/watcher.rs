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
    chain_name: String,
}

impl Watcher {
    pub async fn new(
        rpc_url: String,
        contract_address: String,
        chain_type: ChainType,
        db: Arc<OrderbookProvider>,
        start_block: u64,
    ) -> anyhow::Result<Self> {
        let chain_name = chain_type.name().to_string();

        let chain = match chain_type {
            ChainType::Ethereum(_) => ChainWatcher::Ethereum(
                EthereumChain::new(rpc_url, contract_address, db, start_block).await?,
            ),
            ChainType::Starknet(_) => ChainWatcher::Starknet(
                StarknetChain::new(rpc_url, contract_address, db, start_block).await?,
            ),
        };

        Ok(Self { chain, chain_name })
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        info!("Starting {} watcher service", self.chain_name);

        match &mut self.chain {
            ChainWatcher::Ethereum(chain) => chain.start().await,
            ChainWatcher::Starknet(chain) => chain.start().await,
        }
    }

    pub fn chain_name(&self) -> &str {
        &self.chain_name
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
        let db_url = "postgresql://postgres:e4cqtvu2sHlmwEuy5wSG2ZkINrnxyLNSWpLikE8szXPly4X2NqWfkFKp48y3KKQn@162.55.81.185:3129/postgres";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;
        let db = Arc::new(OrderbookProvider::new(pool));

        let mut chain = EthereumChain::new(
            "https://base-sepolia.drpc.org".to_string(),
            "0xe80CF7Ae2E3Cb8851C8F289bA4d622Cf7B6be5a8".to_string(),
            db,
            29182503,
        )
        .await?;

        chain.start().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_base_chain() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        let db_url = "postgres://king:mangarock@localhost:5432/wallet_db";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;
        let db = Arc::new(OrderbookProvider::new(pool));

        let mut watcher = Watcher::new(
            "https://base.llamarpc.com".to_string(),
            "0x1234567890123456789012345678901234567890".to_string(),
            ChainType::Ethereum("base".to_string()),
            db,
            1000000,
        )
        .await?;

        watcher.start().await?;
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
