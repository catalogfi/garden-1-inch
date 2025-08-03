use crate::{
    chains::{ethereum::EthereumChain, starknet::StarknetChain, traits::Chain},
    orderbook::provider::OrderbookProvider,
    types::ChainType,
    watchers::factory::ChainWatcher,
};
use alloy::{
    json_abi::JsonAbi,
    providers::Provider,
    rpc::types::{Filter, Log},
};
use anyhow::Ok;
use std::sync::Arc;
use tracing::info;

pub struct EscrowWatcher {
    chain: ChainWatcher,
    chain_name: String,
    chain_id: String,
}

impl EscrowWatcher {
    pub async fn new(
        rpc_url: String,
        contract_address: String,
        chain_type: ChainType,
        db: Arc<OrderbookProvider>,
        start_block: u64,
        abi: JsonAbi,
        chain_id: String,
    ) -> anyhow::Result<Self> {
        let chain_name = chain_type.name().to_string();

        let chain = match chain_type {
            ChainType::Ethereum(_) => ChainWatcher::Ethereum(
                EthereumChain::new(rpc_url, contract_address, db, start_block, abi).await?,
            ),
            ChainType::Starknet(_) => ChainWatcher::Starknet(
                StarknetChain::new(rpc_url, contract_address, db, start_block, abi).await?,
            ),
        };

        Ok(Self {
            chain,
            chain_name,
            chain_id,
        })
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

    pub fn chain_id(&self) -> String {
        self.chain_id.to_string()
    }

    pub async fn get_block_number(&self) -> anyhow::Result<u64> {
        match &self.chain {
            ChainWatcher::Ethereum(chain) => Ok(chain.client.get_block_number().await?),
            ChainWatcher::Starknet(_chain) => {
                // You'll need to implement this for Starknet
                // For now, return a placeholder
                Ok(0)
            }
        }
    }

    pub async fn get_logs(&self, filter: &Filter) -> anyhow::Result<Vec<Log>> {
        match &self.chain {
            ChainWatcher::Ethereum(chain) => Ok(chain.client.get_logs(filter).await?),
            ChainWatcher::Starknet(_chain) => {
                // You'll need to implement this for Starknet
                // For now, return empty vec
                Ok(vec![])
            }
        }
    }

    pub async fn process_log(&self, log: Log) -> anyhow::Result<()> {
        match &self.chain {
            ChainWatcher::Ethereum(chain) => chain.process_log(log).await,
            ChainWatcher::Starknet(_chain) => Ok(()),
        }
    }

    // Add method to get the underlying client for more complex operations
    pub fn get_ethereum_chain(&self) -> Option<&EthereumChain> {
        match &self.chain {
            ChainWatcher::Ethereum(chain) => Some(chain),
            ChainWatcher::Starknet(_) => None,
        }
    }

    pub fn get_starknet_chain(&self) -> Option<&StarknetChain> {
        match &self.chain {
            ChainWatcher::Ethereum(_) => None,
            ChainWatcher::Starknet(chain) => Some(chain),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::{fs, path::Path};

    #[tokio::test]
    async fn test_escrow_base() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        let db_url = "postgresql://postgres:e4cqtvu2sHlmwEuy5wSG2ZkINrnxyLNSWpLikE8szXPly4X2NqWfkFKp48y3KKQn@162.55.81.185:3129/postgres";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;
        let db = Arc::new(OrderbookProvider::new(pool));

        let json_abi = load_abi(Path::new("src/abi/escrow_src.json"))?;

        let mut watcher = EscrowWatcher::new(
            "https://base-sepolia.drpc.org".to_string(),
            "0xe6f15fe59e724643555ba890a79f0437f296d09c".to_string(),
            ChainType::Ethereum("base".to_string()),
            db,
            29196316,
            json_abi,
            "84532".to_string(),
        )
        .await?;

        watcher.start().await?;
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
