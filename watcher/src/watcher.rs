use crate::orderbook::orderbook::OrderbookProvider;
use crate::types::ChainType;
use crate::types::ClientType;
use crate::types::EthereumClient;
use crate::types::StarknetClient;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::rpc::types::Filter;
use alloy::rpc::types::Log;
use starknet::core::types::BlockId;
use starknet::core::types::EmittedEvent;
use starknet::core::types::EventFilter;
use starknet::core::types::Felt;
use starknet::core::types::MaybePendingBlockWithTxs;
use starknet::providers::Provider as StarknetProvider;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::error;
use tracing::info;

const MAX_BLOCK_SPAN: u64 = 1000;

#[derive(Clone, Debug)]
pub struct Watcher {
    pub rpc_url: String,
    pub contracts: Vec<ContractDetails>,
    pub client: ClientType,
    pub chain_type: ChainType,
    pub db: Arc<OrderbookProvider>,
}

#[derive(Clone, Debug)]
pub struct ContractDetails {
    pub address: String,
    pub last_block: Option<u64>,
}

impl Watcher {
    pub async fn start(&self) -> Result<(), anyhow::Error> {
        println!(
            "Starting watcher for {:?} chain on {}",
            self.chain_type, self.rpc_url
        );

        match &self.client {
            ClientType::Ethereum(client) => {
                self.clone().watch_ethereum_events(client).await?;
            }
            ClientType::Starknet(client) => {
                self.watch_starknet_events(client).await?;
            }
        }
        Ok(())
    }

    async fn watch_starknet_events(
        &self,
        _client: &Arc<StarknetClient>,
    ) -> Result<(), anyhow::Error> {
        let poll_interval = Duration::from_secs(5);
        loop {
            if let Err(e) = self.clone().poll_events().await {
                error!("Error polling Ethereum events: {}", e);
            }
            sleep(poll_interval).await;
        }
    }

    async fn watch_ethereum_events(
        &mut self,
        _client: &Arc<EthereumClient>,
    ) -> Result<(), anyhow::Error> {
        let poll_interval = Duration::from_secs(5);

        loop {
            if let Err(e) = self.poll_events().await {
                error!("Error polling Ethereum events: {}", e);
            }
            sleep(poll_interval).await;
        }
    }

    async fn process_ethereum_events(
        &self,
        client: &EthereumClient,
        contract_address: &str,
        from_block: u64,
        to_block: u64,
    ) -> Result<(), anyhow::Error> {
        let address = contract_address.parse::<Address>()?;

        // Create a filter for all events from this contract
        let filter = Filter::new()
            .from_block(from_block)
            .to_block(to_block)
            .address(address);

        let logs = client.get_logs(&filter).await?;

        if logs.is_empty() {
            info!(
                "No events found for contract {} in block range {}-{}",
                contract_address, from_block, to_block
            );
            return Ok(());
        }

        info!(
            events_count = %logs.len(),
            from_block = %from_block,
            to_block = %to_block,
            "found events",
        );

        // Process each event
        for log in logs {
            let block_number = match log.block_number {
                Some(num) => num,
                None => {
                    error!("Event missing block number: {:?}", log);
                    continue;
                }
            };

            let timestamp = self
                .get_ethereum_block_timestamp(client, block_number)
                .await?;

            match self.handle_ethereum_event(log, timestamp).await {
                Ok(_) => info!("Processed event successfully"),
                Err(e) => error!("Failed to process event: {}", e),
            }
        }

        Ok(())
    }

    async fn get_ethereum_block_timestamp(
        &self,
        client: &EthereumClient,
        block_number: u64,
    ) -> Result<u64, anyhow::Error> {
        let block = client.get_block_by_number(block_number.into()).await?;
        match block {
            Some(block) => Ok(block.header.timestamp),
            None => Err(anyhow::anyhow!("Block {} not found", block_number)),
        }
    }

    async fn handle_ethereum_event(&self, log: Log, _timestamp: u64) -> Result<(), anyhow::Error> {
        // TODO: Implement this similar to StarkNet version
        // 1. Parse event data
        // 2. Transform into your domain model
        // 3. Save to database
        // 4. Trigger any downstream actions

        info!("Handling Ethereum event with topics: {:#?}", log.topics());

        Ok(())
    }

    pub async fn poll_events(&mut self) -> Result<(), anyhow::Error> {
        let watcher = self.clone();
        let (latest_block, chain_name) = match &self.client {
            ClientType::Ethereum(client) => (
                client.get_block_number().await?,
                ChainType::Ethereum.to_string(),
            ),
            ClientType::Starknet(client) => (
                client.block_hash_and_number().await?.block_number,
                ChainType::Starknet.to_string(),
            ),
        };

        info!("Latest {} block: {}", chain_name, latest_block);

        for contract in &mut self.contracts {
            let from_block = contract.last_block.unwrap_or(latest_block);
            if from_block >= latest_block {
                info!(
                    "Skipping contract {}: last block {} >= latest block {}",
                    contract.address, from_block, latest_block
                );
                continue;
            }

            info!(
                "Polling {} contract {} from block {} to {}",
                chain_name, contract.address, from_block, latest_block
            );

            match &self.client {
                ClientType::Starknet(client) => {
                    // watcher
                    //     .process_starknet_events(
                    //         client,
                    //         &contract.address,
                    //         from_block,
                    //         latest_block,
                    //     )
                    //     .await?;

                    let mut current_block: u64 = from_block;
                    while current_block < latest_block {
                        let next_block =
                            std::cmp::min(current_block + MAX_BLOCK_SPAN, latest_block);

                        watcher
                            .process_starknet_events(
                                client,
                                &contract.address,
                                current_block,
                                next_block,
                            )
                            .await?;

                        current_block = next_block + 1;
                    }
                }
                ClientType::Ethereum(client) => {
                    // watcher
                    //     .process_ethereum_events(
                    //         client,
                    //         &contract.address,
                    //         from_block,
                    //         latest_block,
                    //     )
                    //     .await?;
                    let mut current_block: u64 = from_block;

                    while current_block < latest_block {
                        let next_block =
                            std::cmp::min(current_block + MAX_BLOCK_SPAN, latest_block);

                        watcher
                            .process_ethereum_events(
                                client,
                                &contract.address,
                                current_block,
                                next_block,
                            )
                            .await?;

                        current_block = next_block + 1;
                    }
                }
            }

            contract.last_block = Some(latest_block);
        }

        Ok(())
    }

    async fn process_starknet_events(
        &self,
        client: &StarknetClient,
        contract_address: &str,
        from_block: u64,
        to_block: u64,
    ) -> Result<(), anyhow::Error> {
        let contract_felt = Felt::from_str(contract_address)?;
        let filter = EventFilter {
            from_block: Some(BlockId::Number(from_block)),
            to_block: Some(BlockId::Number(to_block)),
            keys: None,
            address: Some(contract_felt),
        };

        let mut continuation_token: Option<String> = None;

        loop {
            let events = client
                .get_events(filter.clone(), continuation_token.clone(), 100)
                .await?;

            if events.events.is_empty() {
                info!(
                    "No events found for contract {} in block range {}-{}",
                    contract_address, from_block, to_block
                );
                break;
            }

            info!(
                events_count = %events.events.len(),
                from_block = %from_block,
                to_block = %to_block,
                "found events",
            );

            // Process each batch of events
            for event in events.events {
                let block_number = match event.block_number {
                    Some(num) => num,
                    None => {
                        error!("Event missing block number: {:?}", event);
                        continue;
                    }
                };

                let timestamp = self.get_block_timestamp(client, block_number).await?;

                match self.handle_starknet_event(event, timestamp).await {
                    Ok(_) => info!("Processed event successfully"),
                    Err(e) => error!("Failed to process event: {}", e),
                }
            }

            // Continue pagination if needed
            continuation_token = match events.continuation_token {
                Some(token) => Some(token),
                None => break,
            };
        }

        Ok(())
    }

    async fn get_block_timestamp(
        &self,
        client: &StarknetClient,
        block_number: u64,
    ) -> Result<u64, anyhow::Error> {
        let block = client
            .get_block_with_txs(BlockId::Number(block_number))
            .await?;
        match block {
            MaybePendingBlockWithTxs::Block(block) => Ok(block.timestamp),
            MaybePendingBlockWithTxs::PendingBlock(block) => Ok(block.timestamp),
        }
    }

    async fn handle_starknet_event(
        &self,
        _event: EmittedEvent,
        timestamp: u64,
    ) -> Result<(), anyhow::Error> {
        // TODO: Implement this
        // 1. Parse event data
        // 2. Transform into your domain model
        // 3. Save to database
        // 4. Trigger any downstream actions
        info!("Handling event at block timestamp: {}", timestamp);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::providers::RootProvider;
    use alloy::rpc::client::RpcClient;
    use alloy::transports::http::Http;
    use sqlx::{Pool, Postgres};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_poll_ethereum() -> Result<(), anyhow::Error> {
        tracing_subscriber::fmt::init();

        let eth_url = "https://eth.llamarpc.com";
        let transport = Http::new(eth_url.parse()?);
        let provider: Arc<EthereumClient> =
            Arc::new(RootProvider::new(RpcClient::new(transport, false)));
        let db_url = "postgres://king:mangarock@localhost:5432/wallet_db";
        let pool: Pool<Postgres> = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;

        let watcher = Watcher {
            rpc_url: eth_url.to_string(),
            chain_type: ChainType::Ethereum,
            contracts: vec![ContractDetails {
                address: "0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string(),
                last_block: Some(23038436),
            }],
            client: ClientType::Ethereum(provider),
            db: Arc::new(OrderbookProvider::new(pool)),
        };

        watcher.clone().poll_events().await?;
        Ok(())
    }

    //  #[tokio::test]
    // async fn test_poll_starknet() -> Result<(), anyhow::Error> {
    //     tracing_subscriber::fmt::init();

    //     let eth_url = "https://eth.llamarpc.com";
    //     let transport = Http::new(eth_url.parse()?);
    //     let provider: Arc<EthereumClient> =
    //         Arc::new(RootProvider::new(RpcClient::new(transport, false)));
    //     let db_url = "postgres://king:mangarock@localhost:5432/wallet_db";
    //     let pool: Pool<Postgres> = sqlx::postgres::PgPoolOptions::new()
    //         .max_connections(5)
    //         .connect(db_url)
    //         .await?;

    //     let watcher = Watcher {
    //         rpc_url: eth_url.to_string(),
    //         chain_type: ChainType::Ethereum,
    //         contracts: vec![ContractDetails {
    //             address: "0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string(),
    //             last_block: Some(23038436),
    //         }],
    //         client: ClientType::Ethereum(provider),
    //         db: Arc::new(OrderbookProvider::new(pool)),
    //     };

    //     watcher.clone().poll_events().await?;
    //     Ok(())
    // }
}
