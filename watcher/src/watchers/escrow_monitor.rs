use std::{collections::HashMap, str::FromStr, sync::Arc, time::Duration};

use alloy::{json_abi::JsonAbi, primitives::Address, providers::Provider, rpc::types::Filter};
use tokio::time::interval;
use tracing::{error, info, warn};

use crate::{
    chains::{
        ethereum::{EthereumChain, decode_log_with_abi},
        impls::withdrawal::{ParamOne, WithdrawalEvent},
        traits::Chain,
    },
    config::WatcherConfig,
    orderbook::provider::OrderbookProvider,
};

const ESCROW_MONITOR_INTERVAL: u64 = 5;

pub struct EscrowMonitor {
    db: Arc<OrderbookProvider>,
    chains: HashMap<i64, EthereumChain>,
    escrow_abi: JsonAbi,
}

impl EscrowMonitor {
    pub async fn new(
        db: Arc<OrderbookProvider>,
        config: &WatcherConfig,
        escrow_abi: JsonAbi,
    ) -> anyhow::Result<Self> {
        let mut chains = HashMap::new();

        for evm_config in &config.chains.evm {
            if !evm_config.rpc_url.is_empty() {
                let chain = EthereumChain::new(
                    evm_config.rpc_url.clone(),
                    String::new(),
                    db.clone(),
                    0,
                    escrow_abi.clone(),
                )
                .await?;

                chains.insert(evm_config.chain_id.try_into().unwrap(), chain);
                info!(
                    "Initialized escrow monitor for chain {}",
                    evm_config.chain_id
                );
            }
        }

        Ok(Self {
            db,
            chains,
            escrow_abi,
        })
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        info!("Starting escrow monitor service");

        let mut monitor_interval = interval(Duration::from_secs(ESCROW_MONITOR_INTERVAL));

        loop {
            monitor_interval.tick().await;

            if let Err(e) = self.monitor_escrows().await {
                error!("Error monitoring escrows: {}", e);
            }
        }
    }

    async fn monitor_escrows(&self) -> anyhow::Result<()> {
        let escrow_addresses_by_chain = self
            .db
            .get_escrow_addresses_by_chain()
            .await
            .expect("Unable to get the escrow address by chain");

        if escrow_addresses_by_chain.is_empty() {
            info!("No pending escrows to monitor");
            return Ok(());
        }

        info!(
            "Monitoring {} chains with escrows",
            escrow_addresses_by_chain.len()
        );

        // Monitor each chain's escrows
        for (chain_id, escrow_addresses) in escrow_addresses_by_chain {
            if let Some(chain) = self.chains.get(&chain_id) {
                if let Err(e) = self.monitor_chain_escrows(chain, &escrow_addresses).await {
                    error!("Error monitoring escrows for chain {}: {}", chain_id, e);
                }
            } else {
                warn!("No chain configuration found for chain_id: {}", chain_id);
            }
        }

        Ok(())
    }

    async fn monitor_chain_escrows(
        &self,
        chain: &EthereumChain,
        escrow_addresses: &[String],
    ) -> anyhow::Result<()> {
        info!(
            "Monitoring {} escrow addresses on chain",
            escrow_addresses.len(),
        );

        // Get current block number
        let latest_block = chain.client.get_block_number().await?;
        let from_block = latest_block.saturating_sub(100); // Look back 100 blocks

        // Create addresses for filtering
        let addresses: Result<Vec<Address>, _> = escrow_addresses
            .iter()
            .map(|addr| Address::from_str(addr))
            .collect();

        let addresses = addresses?;

        if addresses.is_empty() {
            return Ok(());
        }

        // Create filter for withdrawal events from these escrow addresses
        let filter = Filter::new()
            .from_block(from_block)
            .to_block(latest_block)
            .address(addresses);

        // Get logs from all escrow addresses
        let logs = chain.client.get_logs(&filter).await?;

        if !logs.is_empty() {
            info!("Found {} logs from escrow addresses", logs.len());
        }

        // Process each log
        for log in logs {
            if let Err(e) = chain.process_log(log).await {
                error!("Error processing withdrawal log: {}", e);
            }
        }

        Ok(())
    }
}

// Enhanced trait for escrow-specific monitoring
#[async_trait::async_trait]
pub trait EscrowMonitorable {
    async fn monitor_specific_escrows(&self, escrow_addresses: &[String]) -> anyhow::Result<()>;
    async fn get_withdrawal_events(
        &self,
        escrow_addresses: &[String],
        from_block: u64,
        to_block: u64,
    ) -> anyhow::Result<Vec<WithdrawalEvent>>;
}

#[async_trait::async_trait]
impl EscrowMonitorable for EthereumChain {
    async fn monitor_specific_escrows(&self, escrow_addresses: &[String]) -> anyhow::Result<()> {
        if escrow_addresses.is_empty() {
            return Ok(());
        }

        let latest_block = self.client.get_block_number().await?;
        let from_block = latest_block.saturating_sub(50); // Check last 50 blocks

        // Create filter for these specific escrow addresses
        let addresses: Result<Vec<Address>, _> = escrow_addresses
            .iter()
            .map(|addr| Address::from_str(addr))
            .collect();

        let addresses = addresses?;

        let filter = Filter::new()
            .from_block(from_block)
            .to_block(latest_block)
            .address(addresses);

        let logs = self.client.get_logs(&filter).await?;

        for log in logs {
            self.process_log(log).await?;
        }

        Ok(())
    }

    async fn get_withdrawal_events(
        &self,
        escrow_addresses: &[String],
        from_block: u64,
        to_block: u64,
    ) -> anyhow::Result<Vec<WithdrawalEvent>> {
        let addresses: Result<Vec<Address>, _> = escrow_addresses
            .iter()
            .map(|addr| Address::from_str(addr))
            .collect();

        let addresses = addresses?;

        let filter = Filter::new()
            .from_block(from_block)
            .to_block(to_block)
            .address(addresses);

        let logs = self.client.get_logs(&filter).await?;
        let mut withdrawal_events = Vec::new();

        for log in logs {
            if let Some((event_name, decoded_event)) = decode_log_with_abi(&self.abi, &log)? {
                if event_name == "Withdrawal" {
                    let body = decoded_event.body;
                    if body.len() >= 2 {
                        if let (
                            alloy::dyn_abi::DynSolValue::FixedBytes(secret, _),
                            alloy::dyn_abi::DynSolValue::FixedBytes(order_hash_bytes, _),
                        ) = (&body[0], &body[1])
                        {
                            let withdrawal_event = WithdrawalEvent {
                                param_one: ParamOne {
                                    secret: alloy::hex::encode(secret),
                                    order_hash: alloy::hex::encode(order_hash_bytes),
                                },
                            };
                            withdrawal_events.push(withdrawal_event);
                        }
                    }
                }
            }
        }

        Ok(withdrawal_events)
    }
}
