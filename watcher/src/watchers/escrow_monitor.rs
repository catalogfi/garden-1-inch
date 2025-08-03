use std::{collections::HashMap, str::FromStr, sync::Arc, time::Duration};

use alloy::{
    dyn_abi::DynSolValue,
    hex,
    json_abi::JsonAbi,
    primitives::Address,
    rpc::types::{Filter, Log},
};
use tokio::time::interval;
use tracing::{error, info, warn};

use crate::{
    chains::ethereum::decode_log_with_abi,
    config::WatcherConfig,
    orderbook::provider::OrderbookProvider,
    types::{ChainType, OrderStatus},
    watchers::escrow::EscrowWatcher,
};

const ESCROW_MONITOR_INTERVAL: u64 = 5;

pub struct EscrowMonitor {
    pub db: Arc<OrderbookProvider>,
    pub chains: HashMap<String, EscrowWatcher>,
    pub escrow_abi: JsonAbi,
    pub completed_orders: Arc<tokio::sync::RwLock<std::collections::HashSet<String>>>,
    pub start_block: u64,
}

impl EscrowMonitor {
    pub async fn new(
        db: Arc<OrderbookProvider>,
        config: &WatcherConfig,
        escrow_abi: JsonAbi,
        start_block: u64,
    ) -> anyhow::Result<Self> {
        let mut chains = HashMap::new();

        // Initialize EVM chains
        for evm_config in &config.chains.evm {
            if !evm_config.rpc_url.is_empty() {
                let chain = EscrowWatcher::new(
                    evm_config.rpc_url.clone(),
                    String::new(),
                    ChainType::Ethereum(evm_config.name.to_string()),
                    db.clone(),
                    start_block,
                    escrow_abi.clone(),
                    evm_config.chain_id.to_string(),
                )
                .await?;

                chains.insert(evm_config.chain_id.to_string(), chain);
                info!(
                    "Initialized escrow monitor for chain {} ({})",
                    evm_config.name, evm_config.chain_id
                );
            }
        }

        Ok(Self {
            db,
            chains,
            escrow_abi,
            completed_orders: Arc::new(tokio::sync::RwLock::new(std::collections::HashSet::new())),
            start_block,
        })
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        info!("🚀 Starting escrow monitor service");

        let mut monitor_interval = interval(Duration::from_secs(ESCROW_MONITOR_INTERVAL));

        loop {
            monitor_interval.tick().await;

            if let Err(e) = self.monitor_escrows().await {
                error!("Error monitoring escrows: {}", e);
            }
        }
    }

    async fn monitor_escrows(&self) -> anyhow::Result<()> {
        // Get active escrow addresses with order hashes from your existing method
        let escrow_data_by_chain = self
            .db
            .get_escrow_addresses_with_order_hashes_by_chain()
            .await
            .map_err(|e| anyhow::anyhow!("Unable to get escrow addresses by chain: {}", e))?;

        if escrow_data_by_chain.is_empty() {
            info!("🟢 No pending escrows to monitor");
            return Ok(());
        }

        info!(
            "Monitoring {} chains with escrows",
            escrow_data_by_chain.len()
        );

        // Monitor each chain's escrows
        for (chain_id, escrow_data) in escrow_data_by_chain {
            if let Some(chain) = self.chains.get(&chain_id) {
                if let Err(e) = self
                    .monitor_chain_escrows(chain, &escrow_data, chain_id.to_string())
                    .await
                {
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
        chain: &EscrowWatcher,
        escrow_data: &[(String, String)], // (escrow_address, order_hash)
        chain_id: String,
    ) -> anyhow::Result<()> {
        let mut active_escrows = Vec::new();

        for (escrow_addr, order_hash) in escrow_data {
            // Skip if marked completed in our local cache
            {
                let completed_orders = self.completed_orders.read().await;
                if completed_orders.contains(order_hash) {
                    info!("Order {} already completed, skipping", order_hash);
                    continue;
                }
            }

            let order_status = self.db.get_order_status(order_hash).await?;

            let should_monitor = match order_status.as_str() {
                "source_settled" => {
                    self.db
                        .is_escrow_destination(order_hash, escrow_addr)
                        .await?
                }
                "destination_settled" => self.db.is_escrow_source(order_hash, escrow_addr).await?,
                "source_filled" | "destination_filled" => true,
                _ => false,
            };

            if should_monitor {
                active_escrows.push((escrow_addr.clone(), order_hash.clone()));
            } else {
                info!(
                    "Order {} escrow {} doesn't need monitoring (status: {})",
                    order_hash, escrow_addr, order_status
                );
            }
        }

        if active_escrows.is_empty() {
            info!("No active escrows to monitor on chain {}", chain_id);
            return Ok(());
        }

        info!(
            "Monitoring {} escrow addresses on chain {}",
            active_escrows.len(),
            chain_id
        );

        let latest_block = match chain.get_block_number().await {
            Ok(block) => block,
            Err(e) => {
                error!("Failed to get latest block for chain {}: {}", chain_id, e);
                return Ok(());
            }
        };

        let addresses: Vec<Address> = active_escrows
            .iter()
            .filter_map(|(addr, _)| Address::from_str(addr).ok())
            .collect();

        if addresses.is_empty() {
            warn!("No valid escrow addresses to monitor on chain {}", chain_id);
            return Ok(());
        }

        let mut from_block = self.start_block;
        let max_spam = 100;

        while from_block <= latest_block {
            let to_block = std::cmp::min(from_block + max_spam - 1, latest_block);

            // info!("Querying blocks {} to {}", from_block, to_block);

            let filter = Filter::new()
                .from_block(from_block)
                .to_block(to_block)
                .address(addresses.clone());

            match chain.get_logs(&filter).await {
                Ok(logs) => {
                    if !logs.is_empty() {
                        info!(
                            "Found {} logs from escrow addresses on chain {} (blocks {}-{})",
                            logs.len(),
                            chain_id,
                            from_block,
                            to_block
                        );
                    }

                    for log in logs {
                        let log_address = format!("0x{}", hex::encode(log.address()));

                        let Some((_, order_hash)) =
                            active_escrows.iter().find(|(addr, _)| *addr == log_address)
                        else {
                            warn!("No matching order found for escrow {}", log_address);
                            continue;
                        };

                        match self
                            .handle_withdrawn_event(
                                chain,
                                &log,
                                &active_escrows,
                                chain_id.to_string(),
                            )
                            .await
                        {
                            Ok(_) => {
                                // Mark order as completed in our tracking
                                let mut completed = self.completed_orders.write().await;
                                completed.insert(order_hash.clone());
                                info!("Successfully processed withdrawal for order {}", order_hash);
                            }
                            Err(e) => {
                                error!("Error processing withdrawal log: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to get logs for chain {} (blocks {}-{}): {}",
                        chain_id, from_block, to_block, e
                    );
                    // Continue with next block range instead of returning
                }
            }

            // Update from_block for next iteration
            from_block = to_block + 1;

            // Small delay between requests to avoid rate limiting
            tokio::time::sleep(Duration::from_millis(200)).await;
        }

        Ok(())
    }

    async fn handle_withdrawn_event(
        &self,
        _chain: &EscrowWatcher,
        log: &Log,
        active_escrows: &[(String, String)],
        chain_id: String,
    ) -> anyhow::Result<()> {
        // Decode the log using the escrow ABI
        let decoded = decode_log_with_abi(&self.escrow_abi, log)
            .map_err(|e| anyhow::anyhow!("Failed to decode log: {}", e))?;

        let (event_name, decoded_event) = match decoded {
            Some(val) => val,
            None => return Err(anyhow::anyhow!("No matching event found in ABI for log")),
        };

        // Verify we're processing the correct event
        if event_name != "Withdrawal" {
            return Err(anyhow::anyhow!(
                "Expected Withdrawal event, got {}",
                event_name
            ));
        }

        let body = decoded_event.body;
        if body.len() != 2 {
            return Err(anyhow::anyhow!(
                "Expected 2 parameters for Withdrawal event, got {}",
                body.len()
            ));
        }

        let escrow_address = format!("0x{}", hex::encode(log.address()));
        let normalized_escrow = self.db.normalize_address(&escrow_address);

        let event_order_hash = match &body[1] {
            DynSolValue::FixedBytes(order_hash_bytes, _) => {
                hex::encode(order_hash_bytes).to_string()
            }
            _ => return Err(anyhow::anyhow!("Parameter 1 should be FixedBytes")),
        };

        let escrow_address = format!("0x{}", hex::encode(log.address()));

        // Find the matching order hash for this escrow address
        let order_hash = active_escrows
            .iter()
            .find(|(addr, _)| self.db.normalize_address(addr) == normalized_escrow)
            .map(|(_, hash)| hash)
            .ok_or_else(|| anyhow::anyhow!("No matching order hash found for escrow address"))?;

        // Get current status before updating
        let current_status = self.db.get_order_status(order_hash).await?;

        let withdrawal_status = self
            .db
            .determine_withdrawal_status(&event_order_hash, &escrow_address)
            .await?;

        // Check if we should directly mark as fulfilled
        let final_status = match (&withdrawal_status, current_status.as_str()) {
            (OrderStatus::SourceSettled, "destination_settled") => OrderStatus::FulFilled,
            (OrderStatus::DestinationSettled, "source_settled") => OrderStatus::FulFilled,
            _ => withdrawal_status,
        };

        self.db
            .update_order_status(order_hash, final_status.clone())
            .await
            .map_err(|e| anyhow::anyhow!("Failed to update status: {}", e))?;

        // Mark as completed if fulfilled
        if final_status.to_string() == "fulfilled" {
            let mut completed = self.completed_orders.write().await;
            completed.insert(order_hash.clone());
            info!("Order {} is now FULFILLED - marked as complete", order_hash);
        }

        info!(
            "Successfully processed withdrawal for order hash: {} on chain {} with status: {}",
            event_order_hash,
            chain_id,
            final_status.to_string()
        );

        Ok(())
    }
}
