use crate::orderbook::provider::OrderbookProvider;
use alloy::json_abi::JsonAbi;
use async_trait::async_trait;
use std::fmt::Debug;
use std::sync::Arc;

#[async_trait]
pub trait Chain: Send + Sync + Debug {
    type Event;

    async fn new(
        rpc_url: String,
        contract_address: String,
        db: Arc<OrderbookProvider>,
        start_block: u64,
        abi: JsonAbi,
    ) -> anyhow::Result<Self>
    where
        Self: Sized;

    async fn start(&mut self) -> anyhow::Result<()>;
    async fn poll_events(&mut self) -> anyhow::Result<()>;
    async fn get_block_timestamp(&self, block_number: u64) -> anyhow::Result<u64>;
    fn get_polling_interval(&self) -> u64;
    async fn process_log(&self, event: Self::Event) -> anyhow::Result<()>;
}
