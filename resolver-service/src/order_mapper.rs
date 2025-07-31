use std::{collections::{HashMap, HashSet}, time::Duration};
use anyhow::Result;
use moka::future::Cache;
use tokio::time::sleep;

use crate::oneinch::orders::{ActiveOrdersOutput, ActiveOrdersParams, OrderStatus, OrdersClient};
use crate::resolver::Resolver;

#[derive(Debug)]
pub struct OrderAction {
    pub order_id: String,
    pub action_type: ActionType,
    pub order: ActiveOrdersOutput,
}

#[derive(Debug, Clone)]
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
}

impl OrderMapperBuilder {
    pub fn new() -> Self {
        Self {
            order_client: None,
            chain_resolvers: HashMap::new(),
            supported_chains: HashSet::new(),
            supported_assets: HashMap::new(),
            poll_interval: Duration::from_secs(5),
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
        })
    }
}

pub struct OrderMapper {
    pub chain_resolvers: HashMap<String, Box<dyn Resolver + Send + Sync>>, // Key by chain_id
    pub supported_chains: HashSet<String>,
    pub supported_assets: HashMap<String, HashSet<String>>, // chain_id -> supported assets
    pub processing_orders: Cache<String, bool>, // Track orders being processed
    pub order_client: OrdersClient,
    pub poll_interval: Duration,
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
            if let Err(e) = self.process_orders().await {
                tracing::error!("Error processing orders: {}", e);
            }
            sleep(self.poll_interval).await;
        }
    }

    async fn execute_action(&self, order: &ActiveOrdersOutput, action_type: ActionType, resolver: &Box<dyn Resolver + Send + Sync>) -> Result<()> {
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

    async fn process_orders(&mut self) -> Result<()> {
        let orders = self.order_client.get_active_orders(ActiveOrdersParams::new()).await?;
        
        for order in orders.items {
            let order_id = order.order_hash.clone();
            
            // Skip if already processing
            if self.processing_orders.get(&order_id).await.is_some() {
                tracing::debug!(order_id=?order_id, "Order already being processed, skipping");
                continue;
            }

            // Check if we support the asset on both chains
            if self.is_supported_order(&order) {
                tracing::info!(order_id=?order_id, "Processing supported order");

                let (source_action_type, destination_action_type) = self.determine_action(&order);

                // Get resolvers with better error handling
                let source_chain_resolver = self.chain_resolvers.get(&order.src_chain_id.to_string())
                    .ok_or_else(|| anyhow::anyhow!("Source chain resolver not found for chain {}", order.src_chain_id))?;
                
                let destination_chain_resolver = self.chain_resolvers.get(&order.dst_chain_id.to_string())
                    .ok_or_else(|| anyhow::anyhow!("Destination chain resolver not found for chain {}", order.dst_chain_id))?;
                
                // Mark as processing before executing actions
                self.processing_orders.insert(order_id.clone(), true).await;
                
                // Execute source chain action
                match self.execute_action(&order, source_action_type, source_chain_resolver).await {
                    Ok(_) => {
                        tracing::info!(order_id=?order_id, src_chain_id=?order.src_chain_id, "Executed action on source chain resolver");
                    }
                    Err(e) => {
                        // Remove from processing if failed
                        self.processing_orders.invalidate(&order_id).await;
                        return Err(anyhow::anyhow!("Failed to execute action on source chain resolver: {}", e));
                    }
                }
                
                // Execute destination chain action
                match self.execute_action(&order, destination_action_type, destination_chain_resolver).await {
                    Ok(_) => {
                        tracing::info!(order_id=?order_id, dst_chain_id=?order.dst_chain_id, "Executed action on destination chain resolver");
                    }
                    Err(e) => {
                        // Remove from processing if failed
                        self.processing_orders.invalidate(&order_id).await;
                        return Err(anyhow::anyhow!("Failed to execute action on destination chain resolver: {}", e));
                    }
                }
            } else {
                tracing::debug!(order_id=?order_id, "Order not supported, skipping");
            }
        }
        Ok(())
    }

    fn extract_chain_id(&self, order: &ActiveOrdersOutput) -> Option<(String, String)> {
        Some((order.src_chain_id.to_string(), order.dst_chain_id.to_string()))
    }  

    fn is_supported_order(&self, order: &ActiveOrdersOutput) -> bool {
        let (src_chain_id, dst_chain_id) = self.extract_chain_id(order).unwrap();
        self.supported_chains.contains(&src_chain_id) &&
        self.supported_chains.contains(&dst_chain_id) &&
        self.supported_assets.get(&src_chain_id).map_or(false, |assets| assets.contains(&order.maker_asset)) &&
        self.supported_assets.get(&dst_chain_id).map_or(false, |assets| assets.contains(&order.taker_asset))
    }

    fn determine_action(&self, order: &ActiveOrdersOutput) -> (ActionType, ActionType) {
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
        }
    }
}