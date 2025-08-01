use crate::{chains::traits::Chain, orderbook::provider::OrderbookProvider};
use alloy::{
    network::Ethereum,
    providers::{Provider as EthereumProvider, RootProvider},
    rpc::{
        client::RpcClient,
        types::{Filter, Log},
    },
    transports::http::Http,
};
use async_trait::async_trait;
use std::{str::FromStr, sync::Arc, time::Duration};
use tokio::time::sleep;
use tracing::{error, info};

const MAX_BLOCK_SPAN: u64 = 1000;
const POLLING_INTERVAL: u64 = 5;

#[derive(Debug)]
pub struct EthereumChain {
    client: Arc<RootProvider<Ethereum>>,
    contract_address: String,
    _db: Arc<OrderbookProvider>,
    last_block: Option<u64>,
    start_block: u64,
}

#[async_trait]
impl Chain for EthereumChain {
    type Event = Log;

    async fn new(
        rpc_url: String,
        contract_address: String,
        db: Arc<OrderbookProvider>,
        start_block: u64,
    ) -> anyhow::Result<Self> {
        let transport = Http::new(rpc_url.parse()?);
        let provider = RootProvider::new(RpcClient::new(transport, false));
        let client = Arc::new(provider);

        Ok(Self {
            client,
            contract_address,
            _db: db,
            last_block: None,
            start_block,
        })
    }

    async fn start(&mut self) -> anyhow::Result<()> {
        info!(
            "Starting Ethereum watcher for contract: {}",
            self.contract_address
        );

        loop {
            if let Err(e) = self.poll_events().await {
                error!("Error polling Ethereum events: {}", e);
            }
            sleep(Duration::from_secs(self.get_polling_interval())).await;
        }
    }

    async fn poll_events(&mut self) -> anyhow::Result<()> {
        let latest_block = self.client.get_block_number().await?;
        let from_block = self.last_block.unwrap_or(self.start_block);

        info!("Latest block: {}", latest_block);

        if from_block >= latest_block {
            info!(
                "Skipping contract {}: last block {} >= latest block {}",
                self.contract_address, from_block, latest_block
            );
            return Ok(());
        }

        info!(
            "Polling contract {} from block {} to {}",
            self.contract_address, from_block, latest_block
        );

        let mut current_block = from_block;
        while current_block < latest_block {
            let next_block = std::cmp::min(current_block + MAX_BLOCK_SPAN, latest_block);

            let filter = Filter::new()
                .from_block(current_block)
                .to_block(next_block)
                .address(alloy::primitives::Address::from_str(
                    &self.contract_address,
                )?);

            let logs = self.client.get_logs(&filter).await?;

            for log in logs {
                self.process_log(log).await?;
            }

            current_block = next_block + 1;
        }

        self.last_block = Some(latest_block);
        Ok(())
    }

    async fn get_block_timestamp(&self, block_number: u64) -> anyhow::Result<u64> {
        let block = self.client.get_block(block_number.into()).await?;
        if block.is_none() {
            return Err(anyhow::anyhow!("Block not found: {}", block_number));
        }
        let block = block.unwrap();
        Ok(block.header.timestamp)
    }

    fn get_polling_interval(&self) -> u64 {
        POLLING_INTERVAL
    }

    async fn process_log(&self, log: Log) -> anyhow::Result<()> {
        info!("Processing log: {:#?}", log);
        Ok(())
    }
}
