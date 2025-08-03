use std::{collections::{HashMap, HashSet}, time::{Duration, SystemTime}};
use anyhow::Result;
use moka::future::Cache;
use tokio::{time::sleep, fs};

use crate::{oneinch::orders::{ActiveOrderOutput, ActiveOrdersParams, OrderDetail, OrderStatus, OrdersClient}, resolver::Resolver};

#[derive(Debug)]
pub struct OrderAction {
    pub order_id: String,
    pub action_type: ActionType,
    pub order: OrderDetail,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActionType {
    DeploySrcEscrow,
    DeployDestEscrow,
    WidthdrawSrcEscrow,
    WidthdrawDestEscrow,
    ArbitraryCalls,
    NoOp,
}

pub struct OrderMapperBuilder {
    order_client: Option<OrdersClient>,
    chain_resolvers: HashMap<u64, Box<dyn Resolver + Send + Sync>>,
    supported_chains: HashSet<u64>,
    supported_assets: HashMap<u64, HashSet<String>>,
    poll_interval: Duration,
    action_ttl: Duration,
}

impl OrderMapperBuilder {
    pub fn new() -> Self {
        Self {
            order_client: None,
            chain_resolvers: HashMap::new(),
            supported_chains: HashSet::new(),
            supported_assets: HashMap::new(),
            poll_interval: Duration::from_secs(5),
            action_ttl: Duration::from_secs(300), // Default 5 minutes TTL
        }
    }

    /// Set the orders client
    pub fn with_order_client(mut self, order_client: OrdersClient) -> Self {
        self.order_client = Some(order_client);
        self
    }

    /// Add a chain resolver
    pub fn add_chain_resolver(mut self, chain_id: u64, resolver: Box<dyn Resolver + Send + Sync>) -> Self {
        self.chain_resolvers.insert(chain_id.clone(), resolver);
        self.supported_chains.insert(chain_id);
        self
    }

    /// Add multiple supported assets for a specific chain
    pub fn add_supported_assets(mut self, chain_id: u64, assets: Vec<String>) -> Self {
        for asset in assets {
            self.supported_assets.entry(chain_id).or_insert_with(HashSet::new).insert(asset);
        }
        self
    }

    /// Set the poll interval
    pub fn with_poll_interval(mut self, poll_interval: Duration) -> Self {
        self.poll_interval = poll_interval;
        self
    }

    /// Set the action TTL for reprocessing
    pub fn with_action_ttl(mut self, action_ttl: Duration) -> Self {
        self.action_ttl = action_ttl;
        self
    }

    /// Build the OrderMapper
    pub fn build(self) -> Result<OrderMapper> {
        let order_client = self.order_client.ok_or(anyhow::anyhow!("OrderClient must be set"))?;
        
        if self.chain_resolvers.is_empty() {
            return Err(anyhow::anyhow!("At least one chain resolver must be added"));
        }

        Ok(OrderMapper {
            chain_resolvers: self.chain_resolvers,
            supported_chains: self.supported_chains,
            supported_assets: self.supported_assets,
            processing_orders: Cache::new(1000),
            order_client,
            poll_interval: self.poll_interval,
            action_ttl: self.action_ttl,
        })
    }
}

pub struct OrderMapper {
    pub chain_resolvers: HashMap<u64, Box<dyn Resolver + Send + Sync>>, // Key by chain_id
    pub supported_chains: HashSet<u64>, // Key by chain_id
    pub supported_assets: HashMap<u64, HashSet<String>>, // chain_id -> supported assets
    pub processing_orders: Cache<String, (ActionType, SystemTime)>, // Track last processed action and timestamp
    pub order_client: OrdersClient,
    pub poll_interval: Duration,
    pub action_ttl: Duration, // TTL for action reprocessing
}

impl OrderMapper {
    pub fn builder() -> OrderMapperBuilder {
        OrderMapperBuilder::new()
    }
    
    pub async fn run(&mut self) {
        tracing::info!("OrderMapper started with {} supported chains", self.supported_chains.len());
        
        // Load order hashes from file
        if let Err(e) = self.load_order_hashes_from_file().await {
            tracing::warn!("Failed to load order hashes from file: {}", e);
        }
        
        for (chain_id, assets) in &self.supported_assets {
            tracing::info!(chain_id=?chain_id, assets_count=assets.len(), "Chain supports assets");
        }
        
        loop {
            tracing::info!("Processing orders");
            if let Err(e) = self.discover_and_track_orders().await {
                tracing::error!("Error discovering orders: {}", e);
            }
            
            if let Err(e) = self.process_tracked_orders().await {
                tracing::error!("Error processing tracked orders: {}", e);
            }
            
            sleep(self.poll_interval).await;
        }
    }

    async fn execute_action(&self, order: &OrderDetail, action_type: ActionType, resolver: &Box<dyn Resolver + Send + Sync>) -> Result<()> {
        let action = OrderAction {
            order_id: order.order_hash.clone(),
            action_type: action_type.clone(),
            order: order.clone(),
        };
        tracing::info!("Executing action: {:?} for order: {:?}", action_type, order.order_hash);
        match action_type {
            ActionType::DeploySrcEscrow => {
                tracing::debug!(order_id=?order.order_hash, "Executing deploy escrow action");
                resolver.deploy_src_escrow(&action).await
            }
            ActionType::DeployDestEscrow => {
                tracing::debug!(order_id=?order.order_hash, "Executing deploy escrow action");
                resolver.deploy_dest_escrow(&action).await
            }
            ActionType::WidthdrawSrcEscrow => {
                tracing::debug!(order_id=?order.order_hash, "Executing widthdraw src escrow action");
                resolver.widthdraw_src_escrow(&action).await
            }
            ActionType::WidthdrawDestEscrow => {
                tracing::debug!(order_id=?order.order_hash, "Executing widthdraw dest escrow action");
                resolver.widthdraw_dest_escrow(&action).await
            }
            ActionType::ArbitraryCalls => {
                tracing::debug!(order_id=?order.order_hash, "Executing arbitrary calls action");
                resolver.arbitrary_calls(&action).await
            }
            ActionType::NoOp => {
                tracing::debug!(order_id=?order.order_hash, "No action needed");
                Ok(())
            }
        }
    }

    async fn should_process_action(&self, order_id: &str, action_type: &ActionType) -> Result<bool> {
        if let Some((last_action, timestamp)) = self.processing_orders.get(order_id).await {
            let time_since_last = SystemTime::now().duration_since(timestamp)?;
            
            // If same action and within TTL window, skip
            if last_action == *action_type && time_since_last < self.action_ttl {
                tracing::debug!(
                    order_id=?order_id, 
                    action_type=?action_type, 
                    time_since_last=?time_since_last,
                    "Skipping already processed action within TTL"
                );
                return Ok(false);
            }
            
            // If different action, always process (status change)
            if last_action != *action_type {
                tracing::info!(
                    order_id=?order_id, 
                    last_action=?last_action,
                    new_action=?action_type,
                    "Processing new action type for order"
                );
            }
        }
        
        Ok(true)
    }

    async fn discover_and_track_orders(&mut self) -> Result<()> {
        let orders = self.order_client.get_active_orders(ActiveOrdersParams::new()).await?;
        // self.processing_orders.insert("0x1d4e77a02a589a6f7d305a47003f96fce74dcaeeacc55e91427c247c6c2277bc".to_string(), (ActionType::DeployDestEscrow, SystemTime::now())).await;
        tracing::info!("Discovering {} active orders", orders.items.len());
        
        for order in orders.items {
            let order_id = order.order_hash.clone();
            // Get detailed order information to check if supported
            let order_detail = match self.order_client.get_order_by_hash(&order_id).await {
                Ok(detail) => detail,
                Err(e) => {
                    tracing::error!(order_id=?order_id, error=?e, "Failed to get order detail for discovery");
                    continue;
                }
            };
            
            // Check if we support the asset on both chains
            if self.is_supported_order(&order_detail) {
                tracing::info!(order_id=?order_id, "Adding supported order to tracking");
                self.processing_orders.insert(order_id, (ActionType::NoOp, SystemTime::now())).await;
                // self.processing_orders.insert("0x1d4e77a02a589a6f7d305a47003f96fce74dcaeeacc55e91427c247c6c2277bc".to_string(), (ActionType::DeployDestEscrow, SystemTime::now())).await;
            } else {
                tracing::debug!(order_id=?order_id, "Order not supported, skipping");
            }
        }
        Ok(())
    }

    async fn process_tracked_orders(&mut self) -> Result<()> {
        tracing::info!("Processing tracked orders");
        let mut tracked_order_hashes = Vec::new();
        for (key, value) in self.processing_orders.iter() {
            tracked_order_hashes.push(key.to_string());
        } 

        tracing::info!("Processing {} tracked orders", tracked_order_hashes.len());
        
        for order_hash in tracked_order_hashes {
            tracing::info!("Processing tracked order: {:?}", order_hash);
            
            // Get detailed order information by order_hash
            let order_detail = match self.order_client.get_order_by_hash(&order_hash).await {
                Ok(detail) => detail,
                Err(e) => {
                    tracing::error!(order_id=?order_hash, error=?e, "Failed to get order detail");
                    // Remove from tracking if order not found
                    self.processing_orders.remove(&order_hash).await;
                    continue;
                }
            };

            tracing::warn!("order status: {:?}", order_detail.status);

            let (source_action_type, destination_action_type) = self.determine_action(&order_detail);

            // Check if we should process source action
            let should_process_source = self.should_process_action(&order_hash, &source_action_type).await?;
            let should_process_dest = self.should_process_action(&order_hash, &destination_action_type).await?;

            if !should_process_source && !should_process_dest {
                tracing::debug!(order_id=?order_hash, "Skipping order - no new actions to process");
                continue;
            }

            // Get resolvers with better error handling
            let source_chain_resolver = self.chain_resolvers.get(&order_detail.src_chain_id)
                .ok_or_else(|| anyhow::anyhow!("Source chain resolver not found for chain {}", order_detail.src_chain_id))?;
            
            let destination_chain_resolver = self.chain_resolvers.get(&order_detail.dst_chain_id)
                .ok_or_else(|| anyhow::anyhow!("Destination chain resolver not found for chain {}", order_detail.dst_chain_id))?;
            
            // Process source action if needed
            if should_process_source {
                match self.execute_action(&order_detail, source_action_type.clone(), source_chain_resolver).await {
                    Ok(_) => {
                        tracing::info!(order_id=?order_hash, src_chain_id=?order_detail.src_chain_id, "Executed action on source chain resolver");
                        // Cache the processed action
                        self.processing_orders.insert(order_hash.clone(), (source_action_type.clone(), SystemTime::now())).await;
                    }
                    Err(e) => {
                        tracing::error!(order_id=?order_hash, error=?e, "Failed to execute action on source chain resolver");
                        continue;
                    }
                }
            }
            
            // Process destination action if needed
            if should_process_dest {
                match self.execute_action(&order_detail, destination_action_type.clone(), destination_chain_resolver).await {
                    Ok(_) => {
                        tracing::info!(order_id=?order_hash, dst_chain_id=?order_detail.dst_chain_id, "Executed action on destination chain resolver");
                        // Cache the processed action (use destination action for caching)
                        self.processing_orders.insert(order_hash.clone(), (destination_action_type.clone(), SystemTime::now())).await;
                    }
                    Err(e) => {
                        tracing::error!(order_id=?order_hash, error=?e, "Failed to execute action on destination chain resolver");
                        continue;
                    }
                }
            }
        }
        Ok(())
    }


    fn is_supported_order(&self, order: &OrderDetail) -> bool {
        tracing::info!("supported_chains: {:?}", self.supported_chains);
        tracing::info!("supported_assets: {:?}", self.supported_assets);
        self.supported_chains.contains(&order.src_chain_id) &&
        self.supported_chains.contains(&order.dst_chain_id) &&
        self.supported_assets.get(&order.src_chain_id).map_or(false, |assets| assets.contains(&order.maker_asset)) &&
        self.supported_assets.get(&order.dst_chain_id).map_or(false, |assets| assets.contains(&order.taker_asset))
    }

    async fn load_order_hashes_from_file(&mut self) -> Result<()> {
        let file_path = "order_hashes.txt";
        
        match tokio::fs::read_to_string(file_path).await {
            Ok(contents) => {
                let mut loaded_count = 0;
                for line in contents.lines() {
                    let order_hash = line.trim();
                    if !order_hash.is_empty() && order_hash.starts_with("0x") {
                        tracing::info!("Loading order hash from file: {}", order_hash);
                        self.processing_orders.insert(order_hash.to_string(), (ActionType::NoOp, SystemTime::now())).await;
                        loaded_count += 1;
                    } else if !order_hash.is_empty() {
                        tracing::warn!("Skipping invalid order hash format: {}", order_hash);
                    }
                }
                tracing::info!("Loaded {} order hashes from file: {}", loaded_count, file_path);
                Ok(())
            }
            Err(e) => {
                tracing::warn!("Could not read order hashes file {}: {}", file_path, e);
                Err(anyhow::anyhow!("Failed to read order hashes file: {}", e))
            }
        }
    }

    fn determine_action(&self, order: &OrderDetail) -> (ActionType, ActionType) {
        tracing::info!("Determining action for order: {:?}", order.order_hash);
        match order.status {
            OrderStatus::Unmatched => (ActionType::DeploySrcEscrow, ActionType::NoOp),
            OrderStatus::SourceFilled => (ActionType::NoOp, ActionType::DeployDestEscrow),
            OrderStatus::DestinationFilled => (ActionType::WidthdrawSrcEscrow, ActionType::WidthdrawDestEscrow),
            OrderStatus::SourceWithdrawPending => (ActionType::WidthdrawSrcEscrow, ActionType::NoOp),
            OrderStatus::DestinationWithdrawPending => (ActionType::NoOp, ActionType::WidthdrawDestEscrow),
            OrderStatus::SourceSettled => (ActionType::NoOp, ActionType::WidthdrawDestEscrow),
            OrderStatus::DestinationSettled => (ActionType::NoOp, ActionType::NoOp),
            OrderStatus::Expired => (ActionType::ArbitraryCalls, ActionType::ArbitraryCalls),
            OrderStatus::SourceCanceled => (ActionType::NoOp, ActionType::NoOp),
            OrderStatus::DestinationCanceled => (ActionType::NoOp, ActionType::ArbitraryCalls),
            OrderStatus::DestinationRefunded => (ActionType::NoOp, ActionType::ArbitraryCalls),
            OrderStatus::SourceRefunded => (ActionType::NoOp, ActionType::NoOp),
            OrderStatus::FinalityConfirmed => (ActionType::NoOp, ActionType::NoOp),
        }
    }
}