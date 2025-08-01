use std::{collections::{HashMap, HashSet}, time::{Duration, SystemTime}};
use anyhow::Result;
use moka::future::Cache;
use tokio::time::sleep;

use crate::oneinch::orders::{ActiveOrdersParams, OrderStatus, OrdersClient, ActiveOrder, OrderDetail};
use crate::resolver::Resolver;

#[derive(Debug)]
pub struct OrderAction {
    pub order_id: String,
    pub action_type: ActionType,
    pub order: ActiveOrder,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActionType {
    DeployEscrow,
    ReleaseFunds,
    RefundFunds,
    NoOp,
}

pub struct OrderMapperBuilder {
    order_client: Option<OrdersClient>,
    chain_resolvers: HashMap<String, Box<dyn Resolver + Send + Sync>>,
    supported_chains: HashSet<String>,
    supported_assets: HashMap<String, HashSet<String>>,
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
    pub fn add_chain_resolver(mut self, chain_id: String, resolver: Box<dyn Resolver + Send + Sync>) -> Self {
        self.chain_resolvers.insert(chain_id.clone(), resolver);
        self.supported_chains.insert(chain_id);
        self
    }

    /// Add multiple supported assets for a specific chain
    pub fn add_supported_assets(mut self, chain_id: String, assets: Vec<String>) -> Self {
        for asset in assets {
            self.supported_assets.entry(chain_id.clone()).or_insert_with(HashSet::new).insert(asset);
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
    pub chain_resolvers: HashMap<String, Box<dyn Resolver + Send + Sync>>, // Key by chain_id
    pub supported_chains: HashSet<String>,
    pub supported_assets: HashMap<String, HashSet<String>>, // chain_id -> supported assets
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
        for (chain_id, assets) in &self.supported_assets {
            tracing::info!(chain_id=?chain_id, assets_count=assets.len(), "Chain supports assets");
        }
        
        loop {
            tracing::info!("Processing orders");
            if let Err(e) = self.process_orders().await {
                tracing::error!("Error processing orders: {}", e);
            }
            sleep(self.poll_interval).await;
        }
    }

    async fn execute_action(&self, order: &ActiveOrder, action_type: ActionType, resolver: &Box<dyn Resolver + Send + Sync>) -> Result<()> {
        let action = OrderAction {
            order_id: order.order_hash.clone(),
            action_type: action_type.clone(),
            order: order.clone(),
        };

        match action_type {
            ActionType::DeployEscrow => {
                tracing::debug!(order_id=?order.order_hash, "Executing deploy escrow action");
                resolver.deploy_escrow(&action).await
            }
            ActionType::ReleaseFunds => {
                tracing::debug!(order_id=?order.order_hash, "Executing release funds action");
                resolver.release_funds(&action).await
            }
            ActionType::RefundFunds => {
                tracing::debug!(order_id=?order.order_hash, "Executing refund funds action");
                resolver.refund_funds(&action).await
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

    async fn process_orders(&mut self) -> Result<()> {
        let orders = self.order_client.get_active_orders(ActiveOrdersParams::new()).await?;
        tracing::info!("Processing {} orders", orders.items.len());
        
        for order in orders.items {
            let order_id = order.order_hash.clone();
            
            // Check if we support the asset on both chains
            if self.is_supported_order(&order) {
                tracing::info!(order_id=?order_id, "Processing supported order");

                // Get detailed order information to determine status
                let order_detail = match self.order_client.get_order_by_hash(&order_id).await {
                    Ok(detail) => detail,
                    Err(e) => {
                        tracing::error!(order_id=?order_id, error=?e, "Failed to get order detail");
                        continue;
                    }
                };

                let (source_action_type, destination_action_type) = self.determine_action(&order_detail);

                // Check if we should process source action
                let should_process_source = self.should_process_action(&order_id, &source_action_type).await?;
                let should_process_dest = self.should_process_action(&order_id, &destination_action_type).await?;

                if !should_process_source && !should_process_dest {
                    tracing::debug!(order_id=?order_id, "Skipping order - no new actions to process");
                    continue;
                }

                // Get resolvers with better error handling
                let source_chain_resolver = self.chain_resolvers.get(&order.src_chain_id.to_string())
                    .ok_or_else(|| anyhow::anyhow!("Source chain resolver not found for chain {}", order.src_chain_id))?;
                
                let destination_chain_resolver = self.chain_resolvers.get(&order.dst_chain_id.to_string())
                    .ok_or_else(|| anyhow::anyhow!("Destination chain resolver not found for chain {}", order.dst_chain_id))?;
                
                // Process source action if needed
                if should_process_source {
                    match self.execute_action(&order, source_action_type.clone(), source_chain_resolver).await {
                        Ok(_) => {
                            tracing::info!(order_id=?order_id, src_chain_id=?order.src_chain_id, "Executed action on source chain resolver");
                            // Cache the processed action
                            self.processing_orders.insert(order_id.clone(), (source_action_type.clone(), SystemTime::now())).await;
                        }
                        Err(e) => {
                            tracing::error!(order_id=?order_id, error=?e, "Failed to execute action on source chain resolver");
                            return Err(anyhow::anyhow!("Failed to execute action on source chain resolver: {}", e));
                        }
                    }
                }
                
                // Process destination action if needed
                if should_process_dest {
                    match self.execute_action(&order, destination_action_type.clone(), destination_chain_resolver).await {
                        Ok(_) => {
                            tracing::info!(order_id=?order_id, dst_chain_id=?order.dst_chain_id, "Executed action on destination chain resolver");
                            // Cache the processed action (use destination action for caching)
                            self.processing_orders.insert(order_id.clone(), (destination_action_type.clone(), SystemTime::now())).await;
                        }
                        Err(e) => {
                            tracing::error!(order_id=?order_id, error=?e, "Failed to execute action on destination chain resolver");
                            return Err(anyhow::anyhow!("Failed to execute action on destination chain resolver: {}", e));
                        }
                    }
                }
            } else {
                tracing::debug!(order_id=?order_id, "Order not supported, skipping");
            }
        }
        Ok(())
    }

    fn is_supported_order(&self, order: &ActiveOrder) -> bool {
        self.supported_chains.contains(&order.src_chain_id.to_string()) &&
        self.supported_chains.contains(&order.dst_chain_id.to_string()) &&
        self.supported_assets.get(&order.src_chain_id.to_string()).map_or(false, |assets| assets.contains(&order.order.maker_asset)) &&
        self.supported_assets.get(&order.dst_chain_id.to_string()).map_or(false, |assets| assets.contains(&order.order.taker_asset))
    }

    fn determine_action(&self, order: &OrderDetail) -> (ActionType, ActionType) {
        match order.status {
            OrderStatus::Unmatched => (ActionType::DeployEscrow, ActionType::NoOp),
            OrderStatus::SourceFilled => (ActionType::NoOp, ActionType::DeployEscrow),
            OrderStatus::DestinationFilled => (ActionType::ReleaseFunds, ActionType::NoOp),
            OrderStatus::SourceWithdrawPending => (ActionType::ReleaseFunds, ActionType::NoOp),
            OrderStatus::DestinationWithdrawPending => (ActionType::NoOp, ActionType::ReleaseFunds),
            OrderStatus::SourceSettled => (ActionType::NoOp, ActionType::ReleaseFunds),
            OrderStatus::DestinationSettled => (ActionType::NoOp, ActionType::NoOp), // marks finality of the order
            OrderStatus::Expired => (ActionType::RefundFunds, ActionType::RefundFunds),
            OrderStatus::SourceCanceled => (ActionType::NoOp, ActionType::RefundFunds),
            OrderStatus::DestinationCanceled => (ActionType::NoOp, ActionType::RefundFunds),
            OrderStatus::DestinationRefunded => (ActionType::NoOp, ActionType::RefundFunds),
            OrderStatus::SourceRefunded => (ActionType::NoOp, ActionType::NoOp), // marks finality of the order
            OrderStatus::FinalityConfirmed => (ActionType::NoOp, ActionType::NoOp), // secrets can be submitted
        }
    }
}