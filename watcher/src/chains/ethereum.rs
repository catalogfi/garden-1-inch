use crate::{chains::traits::Chain, orderbook::provider::OrderbookProvider};
use alloy::{
    dyn_abi::{DecodedEvent, EventExt},
    json_abi::JsonAbi,
    network::AnyNetwork,
    primitives::{B256, LogData},
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

const MAX_BLOCK_SPAN: u64 = 50;
const POLLING_INTERVAL: u64 = 10;

#[derive(Debug)]
pub struct EthereumChain {
    pub client: Arc<RootProvider<AnyNetwork>>,
    pub contract_address: String,
    pub db: Arc<OrderbookProvider>,
    pub last_block: Option<u64>,
    pub start_block: u64,
    pub abi: JsonAbi,
}

#[async_trait]
impl Chain for EthereumChain {
    type Event = Log;

    async fn new(
        rpc_url: String,
        contract_address: String,
        db: Arc<OrderbookProvider>,
        start_block: u64,
        abi: JsonAbi,
    ) -> anyhow::Result<Self> {
        let transport = Http::new(rpc_url.parse()?);
        let provider = RootProvider::new(RpcClient::new(transport, false));
        let client = Arc::new(provider);

        Ok(Self {
            client,
            contract_address,
            db,
            last_block: None,
            start_block,
            abi,
        })
    }

    async fn start(&mut self) -> anyhow::Result<()> {
        info!(
            "🚀 Starting Ethereum watcher for contract: {}",
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

        info!("📦 Latest block: {}", latest_block);

        if from_block >= latest_block {
            info!(
                "Skipping contract {}: last block {} >= latest block {}",
                self.contract_address, from_block, latest_block
            );
            return Ok(());
        }

        info!(
            "🔍 Polling contract {} from block {} to {}",
            self.contract_address, from_block, latest_block
        );

        let mut current_block = from_block;
        while current_block < latest_block {
            let next_block = std::cmp::min(current_block + MAX_BLOCK_SPAN, latest_block);

            if next_block - current_block > MAX_BLOCK_SPAN {
                error!(
                    "Block range too large: {} blocks (max allowed: {})",
                    next_block - current_block,
                    MAX_BLOCK_SPAN
                );
                return Err(anyhow::anyhow!("Block range too large"));
            }

            let filter = Filter::new()
                .from_block(current_block)
                .to_block(next_block)
                .address(alloy::primitives::Address::from_str(
                    &self.contract_address,
                )?);
            match self.client.get_logs(&filter).await {
                Ok(logs) => {
                    for log in logs {
                        self.process_log(log).await?;
                    }
                    current_block = next_block + 1;
                }
                Err(e) => {
                    error!("Error fetching logs: {}", e);
                    if MAX_BLOCK_SPAN > 10 {
                        error!("Reducing block span and retrying...");
                        sleep(Duration::from_secs(1)).await;
                        continue;
                    } else {
                        return Err(e.into());
                    }
                }
            }
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
        // info!("Processing log: {:#?}", log);

        match decode_log_with_abi(&self.abi, &log)? {
            Some((event_name, decoded_event)) => {
                info!("📥  Found event: {}", event_name);

                match event_name.as_str() {
                    "SrcEscrowCreated" => {
                        self.handle_src_escrow_created_event(decoded_event, log)
                            .await?;
                    }

                    "Withdrawal" => {
                        self.handle_withdrawn_event(decoded_event, log).await?;
                    }

                    "DstEscrowCreated" => {
                        self.handle_dst_escrow_created_event(decoded_event, log)
                            .await?;
                    }
                    _ => {
                        info!("Unhandled event type: {}", event_name);
                    }
                }
            }
            None => info!("Could not decode log with provided ABI"),
        }
        Ok(())
    }
}

pub fn decode_log_with_abi(
    abi: &JsonAbi,
    log: &Log,
) -> anyhow::Result<Option<(String, DecodedEvent)>> {
    let topics = log.topics();
    if topics.is_empty() {
        return Err(anyhow::anyhow!("Log has no topics"));
    }

    let selector = topics[0];

    if let Some(event) = abi
        .events()
        .find(|event| B256::from(event.selector()) == selector)
    {
        let log_data = LogData::new_unchecked(topics.to_vec(), log.data().clone().data);
        let decoded = event.decode_log(&log_data)?;
        return Ok(Some((event.name.clone(), decoded)));
    }

    Ok(None)
}
