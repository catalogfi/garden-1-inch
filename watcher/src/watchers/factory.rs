use std::sync::Arc;

use alloy::json_abi::JsonAbi;
use tracing::info;

use crate::{
    chains::{ethereum::EthereumChain, starknet::StarknetChain, traits::Chain},
    orderbook::provider::OrderbookProvider,
    types::ChainType,
};

pub enum ChainWatcher {
    Ethereum(EthereumChain),
    Starknet(StarknetChain),
}

pub struct FactoryWatcher {
    pub chain: ChainWatcher,
    pub chain_name: String,
}

impl FactoryWatcher {
    pub async fn new(
        rpc_url: String,
        contract_address: String,
        chain_type: ChainType,
        db: Arc<OrderbookProvider>,
        start_block: u64,
        abi: &JsonAbi,
    ) -> anyhow::Result<Self> {
        let chain_name = chain_type.name().to_string();

        let chain = match chain_type {
            ChainType::Ethereum(_) => ChainWatcher::Ethereum(
                EthereumChain::new(rpc_url, contract_address, db, start_block, abi.clone()).await?,
            ),
            ChainType::Starknet(_) => ChainWatcher::Starknet(
                StarknetChain::new(rpc_url, contract_address, db, start_block, abi.clone()).await?,
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
    use serde_json::Value;

    use super::*;
    use crate::chains::starknet::StarknetChain;
    
    use std::fs;
    use std::path::Path;
    use std::sync::Arc;

    // Helper function to create test config
    // fn create_test_config() -> WatcherConfig {
    //     WatcherConfig {
    //         core: CoreConfig {
    //             db_url: "postgresql://postgres:e4cqtvu2sHlmwEuy5wSG2ZkINrnxyLNSWpLikE8szXPly4X2NqWfkFKp48y3KKQn@162.55.81.185:3129/postgres".to_string(),
    //             polling_interval: 5000,
    //         },
    //         chains: ChainsConfig {
    //             evm: vec![EvmChainConfig {
    //                 name: "base".to_string(),
    //                 chain_id: 8453,
    //                 rpc_url: "https://base-sepolia.drpc.org".to_string(),
    //                 contract_address: "0xe80CF7Ae2E3Cb8851C8F289bA4d622Cf7B6be5a8".to_string(),
    //                 start_block: 29182503,
    //             }],
    //             starknet: vec![],
    //         },
    //     }
    // }

    // Abstracted test for FactoryWatcher
    #[tokio::test]
    async fn test_base_chain() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        let db_url = "postgresql://postgres:e4cqtvu2sHlmwEuy5wSG2ZkINrnxyLNSWpLikE8szXPly4X2NqWfkFKp48y3KKQn@162.55.81.185:3129/postgres";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;
        let db = Arc::new(OrderbookProvider::new(pool));

        let factory_abi: JsonAbi = load_abi(Path::new("src/abi/escrow_factory.json"))?;

        let mut watcher = FactoryWatcher::new(
            "https://base-sepolia.drpc.org".to_string(),
            "0xe80CF7Ae2E3Cb8851C8F289bA4d622Cf7B6be5a8".to_string(),
            ChainType::Ethereum("base".to_string()),
            db,
            29191086,
            &factory_abi,
        )
        .await?;

        watcher.start().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_monad_chain() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        let db_url = "postgresql://postgres:e4cqtvu2sHlmwEuy5wSG2ZkINrnxyLNSWpLikE8szXPly4X2NqWfkFKp48y3KKQn@162.55.81.185:3129/postgres";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;
        let db = Arc::new(OrderbookProvider::new(pool));

        let factory_abi: JsonAbi = load_abi(Path::new("src/abi/escrow_factory.json"))?;

        let mut watcher = FactoryWatcher::new(
            "https://testnet-rpc.monad.xyz".to_string(),
            "0x3cbF8736FB94c4aF293aAbF601dAecBdba9f8643".to_string(),
            ChainType::Ethereum("monad".to_string()),
            db,
            28965705,
            &factory_abi,
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

        let json_abi: JsonAbi = load_abi(Path::new("src/abi/escrow_factory.json"))?;

        let mut chain = StarknetChain::new(
            "https://rpc.starknet.lava.build:443".to_string(),
            "0x04718f5a0Fc34cC1AF16A1cdee98fFB20C31f5cD61D6Ab07201858f4287c938D".to_string(),
            db,
            1661663,
            json_abi,
        )
        .await?;

        chain.start().await?;
        Ok(())
    }

    fn load_abi(path: &Path) -> anyhow::Result<JsonAbi> {
        let abi_content = fs::read_to_string(path)?;
        let full_json: Value = serde_json::from_str(&abi_content)?;

        let abi_array = full_json
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Missing 'abi' field in contract artifact"))?;

        let json_abi: JsonAbi = serde_json::from_value(Value::Array(abi_array.clone()))?;

        Ok(json_abi)
    }
}
