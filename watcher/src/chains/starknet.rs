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
            "🚀 Starting Starknet watcher for contract: {}",
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

        info!("📦 Latest block: {}", latest_block);

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

            info!(
                "🔍 Fetching events from block {} to block {}",
                current_block, next_block
            );

            let contract_felt = Felt::from_str(&self.contract_address)?;
            let filter = EventFilter {
                from_block: Some(BlockId::Number(current_block)),
                to_block: Some(BlockId::Number(next_block)),
                address: Some(contract_felt),
                keys: None,
            };

            let mut continuation_token: Option<String> = None;
            let mut total_events_in_range = 0;

            loop {
                let events = self
                    .client
                    .get_events(filter.clone(), continuation_token.clone(), 100)
                    .await?;

                if events.events.is_empty() {
                    break;
                }

                total_events_in_range += events.events.len();

                info!(
                    "📦 Found {} events in this batch (total so far: {})",
                    events.events.len(),
                    total_events_in_range
                );

                // Print detailed information about each event
                for (i, event) in events.events.iter().enumerate() {
                    info!("\n=== EVENT #{} ===", i + 1);
                    info!("Block Number: {:?}", event.block_number);
                    info!("Block Hash: {:?}", event.block_hash);
                    info!("Transaction Hash: {:?}", event.transaction_hash);
                    info!("From Address: {:?}", event.from_address);

                    info!("Keys ({} total):", event.keys.len());
                    for (key_idx, key) in event.keys.iter().enumerate() {
                        info!("  Key[{}]: {}", key_idx, key);
                        info!("  Key[{}] (hex): {:#x}", key_idx, key);
                    }

                    info!("Data ({} total):", event.data.len());
                    for (data_idx, data) in event.data.iter().enumerate() {
                        info!("  Data[{}]: {}", data_idx, data);
                        info!("  Data[{}] (hex): {:#x}", data_idx, data);
                    }
                    info!("================\n");
                }

                for event in events.events {
                    self.process_log(event).await?;
                }

                continuation_token = events.continuation_token;
                if continuation_token.is_none() {
                    break;
                }
            }

            if total_events_in_range == 0 {
                info!(
                    "❌ No events found in block range {} to {}",
                    current_block, next_block
                );
            } else {
                info!(
                    "✅ Processed {} total events in block range {} to {}",
                    total_events_in_range, current_block, next_block
                );
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

        info!("🚀 PROCESSING EVENT: Block {}", block_number);
        let timestamp = self.get_block_timestamp(block_number).await?;

        info!(
            "Processing Starknet event at block {}, timestamp {}, hash: {:#?}",
            block_number, timestamp, event.block_hash
        );

        // Decode and print event data
        if let Some(event_selector) = event.keys.first() {
            info!("\n🔍 DECODED EVENT:");
            info!("Event Selector: {:#x}", event_selector);

            match event_selector.to_string().as_str() {
                "0xf323845026b2be2da82fc16476961f810f279069ce9e128eabc1023a87ade0" => {
                    info!("Event Type: Created");

                    if event.data.len() >= 5 {
                        let order_hash = &event.data[0];
                        let secret_hash_part1 = &event.data[1];
                        let secret_hash_part2 = &event.data[2];
                        let amount_low = &event.data[3];
                        let amount_high = &event.data[4];

                        info!("📋 Decoded Fields:");
                        info!("  order_hash: {:#x}", order_hash);
                        info!("  secret_hash[0-3]: {:#x}", secret_hash_part1);
                        info!("  secret_hash[4-7]: {:#x}", secret_hash_part2);
                        info!("  amount (low): {} (decimal: {})", amount_low, amount_low);
                        info!(
                            "  amount (high): {} (decimal: {})",
                            amount_high, amount_high
                        );

                        // Calculate full u256 amount
                        let amount_low_u128 = amount_low.to_string().parse::<u128>().unwrap_or(0);
                        let amount_high_u128 = amount_high.to_string().parse::<u128>().unwrap_or(0);

                        if amount_high_u128 == 0 {
                            info!("  💰 Total Amount: {}", amount_low_u128);
                        } else {
                            info!(
                                "  💰 Total Amount: {} + ({} << 128)",
                                amount_low_u128, amount_high_u128
                            );
                        }
                    } else {
                        info!("❌ Insufficient data fields for Created event");
                    }
                }
                _ => {
                    info!("❓ Unknown event selector: {:#x}", event_selector);
                    info!("Data fields:");
                    for (i, data) in event.data.iter().enumerate() {
                        info!("  data[{}]: {:#x} (decimal: {})", i, data, data);
                    }
                }
            }
        }
        info!("=====================================\n");

        Ok(())
    }
}
