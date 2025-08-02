use crate::{chains::traits::Chain, orderbook::provider::OrderbookProvider};
use alloy::json_abi::JsonAbi;
use async_trait::async_trait;
use reqwest::Url;
use starknet::{
    core::types::{BlockId, EmittedEvent, EventFilter, Felt, MaybePendingBlockWithTxs},
    providers::{JsonRpcClient, Provider, jsonrpc::HttpTransport},
};
use std::{str::FromStr, sync::Arc, time::Duration};
use tokio::time::sleep;
use tracing::{error, info};

const MAX_BLOCK_SPAN: u64 = 1000;
const POLLING_INTERVAL: u64 = 5;

#[derive(Debug)]
pub struct StarknetChain {
    client: Arc<JsonRpcClient<HttpTransport>>,
    contract_address: String,
    _db: Arc<OrderbookProvider>,
    last_block: Option<u64>,
    start_block: u64,
}

#[async_trait]
impl Chain for StarknetChain {
    type Event = EmittedEvent;

    async fn new(
        rpc_url: String,
        contract_address: String,
        db: Arc<OrderbookProvider>,
        start_block: u64,
        _abi: JsonAbi,
    ) -> anyhow::Result<Self> {
        let transport = HttpTransport::new(Url::parse(&rpc_url)?);
        let client = Arc::new(JsonRpcClient::new(transport));

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
            "Starting Starknet watcher for contract: {}",
            self.contract_address
        );

        loop {
            if let Err(e) = self.poll_events().await {
                error!("Error polling Starknet events: {}", e);
            }
            sleep(Duration::from_secs(self.get_polling_interval())).await;
        }
    }

    async fn poll_events(&mut self) -> anyhow::Result<()> {
        let latest_block = self.client.block_hash_and_number().await?.block_number;
        let from_block = self.last_block.unwrap_or(self.start_block);

        info!("Latest block: {}", latest_block);

        if from_block >= latest_block {
            info!(
                "Skipping contract {}: last block {} >= latest block {}",
                self.contract_address, from_block, latest_block
            );
            return Ok(());
        }

        let mut current_block = from_block;
        while current_block < latest_block {
            let next_block = std::cmp::min(current_block + MAX_BLOCK_SPAN, latest_block);

            let contract_felt = Felt::from_str(&self.contract_address)?;
            let filter = EventFilter {
                from_block: Some(BlockId::Number(current_block)),
                to_block: Some(BlockId::Number(next_block)),
                address: Some(contract_felt),
                keys: None,
            };

            let mut continuation_token: Option<String> = None;

            loop {
                let events = self
                    .client
                    .get_events(filter.clone(), continuation_token.clone(), 100)
                    .await?;

                if events.events.is_empty() {
                    break;
                }

                info!(
                    events_count = %events.events.len(),
                    from_block = %current_block,
                    to_block = %next_block,
                    "found events",
                );

                for event in events.events {
                    self.process_log(event).await?;
                }

                continuation_token = events.continuation_token;
                if continuation_token.is_none() {
                    break;
                }
            }

            current_block = next_block + 1;
        }

        self.last_block = Some(latest_block);
        Ok(())
    }

    async fn get_block_timestamp(&self, block_number: u64) -> anyhow::Result<u64> {
        let block = self
            .client
            .get_block_with_txs(BlockId::Number(block_number))
            .await?;

        match block {
            MaybePendingBlockWithTxs::Block(block) => Ok(block.timestamp),
            MaybePendingBlockWithTxs::PendingBlock(block) => Ok(block.timestamp),
        }
    }

    fn get_polling_interval(&self) -> u64 {
        POLLING_INTERVAL
    }

    async fn process_log(&self, event: EmittedEvent) -> anyhow::Result<()> {
        let block_number = event
            .block_number
            .ok_or_else(|| anyhow::anyhow!("Event missing block number"))?;

        let timestamp = self.get_block_timestamp(block_number).await?;

        info!(
            "Processing Starknet event at block {}, timestamp {}, hash: {:#?}",
            block_number, timestamp, event.block_hash
        );

        // // Match on event keys to determine event type
        // if let Some(keys) = event.keys.first() {
        //     match keys.to_string().as_str() {
        //         // Add your Starknet event selectors here
        //         // Example:
        //         "0x1234..." => {
        //             self.db
        //                 .handle_escrow_event(
        //                     &event.transaction_hash.to_string(),
        //                     WatcherEventType::SourceEscrowCreated,
        //                     &self.contract_address,
        //                 )
        //                 .await?;
        //         }
        //         // Add more event handlers here
        //         _ => {
        //             info!("Unknown event key: {:?}", keys);
        //         }
        //     }
        // }

        Ok(())
    }
}
