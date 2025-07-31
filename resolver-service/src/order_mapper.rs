use std::{collections::{HashMap, HashSet}, sync::mpsc::{Sender, channel}, time::Duration};
use tokio::time::sleep;

use crate::oneinch::orders::{ActiveOrdersOutput, ActiveOrdersParams, OrdersClient};

#[derive(Debug, Clone)]
pub struct OrderAction {
    pub order_id: String,
    pub action_type: ActionType,
    pub order_data: OrderData, // Full order context
}

#[derive(Debug, Clone)]
pub enum ActionType {
    DeployEscrow,
    ReleaseFunds,
    RefundFunds,
}

// Order Mapper has two jobs 
// 1. Map orders to actions
// 2. Map actions to resolvers

#[derive(Debug, Clone)]
pub struct OrderData {
    pub maker_asset: String,
    pub taker_asset: String,
    pub chain_id: String,
    pub amount: String,
    // Add other necessary fields
}

pub struct OrderMapperBuilder {
    order_client: Option<OrdersClient>,
    chain_resolvers: HashMap<String, Sender<OrderAction>>,
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

    /// Add a chain resolver with its sender
    pub fn add_chain_resolver(mut self, chain_id: String, sender: Sender<OrderAction>) -> Self {
        self.chain_resolvers.insert(chain_id.clone(), sender);
        self.supported_chains.insert(chain_id);
        self
    }

    /// Add a supported asset for a specific chain
    pub fn add_supported_asset(mut self, chain_id: String, asset: String) -> Self {
        self.supported_assets.entry(chain_id.clone()).or_insert_with(HashSet::new).insert(asset);
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
    pub fn build(self) -> Result<OrderMapper, String> {
        let order_client = self.order_client.ok_or("OrderClient must be set")?;
        
        if self.chain_resolvers.is_empty() {
            return Err("At least one chain resolver must be added".to_string());
        }

        Ok(OrderMapper {
            chain_resolvers: self.chain_resolvers,
            supported_chains: self.supported_chains,
            supported_assets: self.supported_assets,
            processing_orders: HashSet::new(),
            order_client,
            poll_interval: self.poll_interval,
        })
    }
}

pub struct OrderMapper {
    pub chain_resolvers: HashMap<String, Sender<OrderAction>>, // Key by chain_id
    pub supported_chains: HashSet<String>,
    pub supported_assets: HashMap<String, HashSet<String>>, // chain_id -> supported assets
    pub processing_orders: HashSet<String>, // Track orders being processed
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
            tracing::info!("Chain {} supports {} assets", chain_id, assets.len());
        }
        
        loop {
            if let Err(e) = self.process_orders().await {
                tracing::error!("Error processing orders: {}", e);
            }
            sleep(self.poll_interval).await;
        }
    }

    async fn process_orders(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let orders = self.order_client.get_active_orders(ActiveOrdersParams::new()).await?;
        
        for order in orders.items {
            let order_id = order.order_hash.clone();
            
            // Skip if already processing
            if self.processing_orders.contains(&order_id) {
                continue;
            }

            // Check if we support this chain and asset
            if let Some(chain_id) = self.extract_chain_id(&order) {
                if self.is_supported_order(&chain_id, &order.order.taker_asset) {
                    if let Some(resolver) = self.chain_resolvers.get(&chain_id) {
                        let chain_id_clone = chain_id.clone();
                        let action = OrderAction {
                            order_id: order_id.clone(),
                            action_type: self.determine_action(&order),
                            order_data: OrderData {
                                maker_asset: order.order.maker_asset,
                                taker_asset: order.order.taker_asset,
                                chain_id,
                                amount: order.order.taking_amount,
                            },
                        };

                        match resolver.send(action) {
                            Ok(_) => {
                                self.processing_orders.insert(order_id.clone());
                                tracing::info!("Sent order {} to resolver for chain {}", order_id, chain_id_clone);
                            }
                            Err(e) => {
                                tracing::error!("Failed to send order {} to resolver: {}", order_id, e);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn extract_chain_id(&self, order: &ActiveOrdersOutput) -> Option<String> {
        // Logic to determine chain from order data
        // This depends on your order structure
        Some("1".to_string()) // Placeholder
    }

    fn is_supported_order(&self, chain_id: &str, asset: &str) -> bool {
        self.supported_chains.contains(chain_id) &&
        self.supported_assets.get(chain_id).map_or(false, |assets| assets.contains(asset))
    }

    fn determine_action(&self, order: &ActiveOrdersOutput) -> ActionType {
        // Logic to determine what action this order needs
        // Based on order status, type, etc.
        ActionType::DeployEscrow // Placeholder
    }

    pub fn mark_order_completed(&mut self, order_id: &str) {
        self.processing_orders.remove(order_id);
    }
}

// Helper function to create a channel for a chain
pub fn create_chain_channel() -> (Sender<OrderAction>, std::sync::mpsc::Receiver<OrderAction>) {
    channel()
}