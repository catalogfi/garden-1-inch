use tokio::sync::mpsc::Receiver;

use crate::order_mapper::{OrderAction, ActionType};

pub struct ResolverContract {
    address: String,
    abi: String,
    provider: String,
}

impl ResolverContract {
    pub fn new(address: String, abi: String, provider: String) -> Self {
        Self {
            address,
            abi,
            provider,
        }
    }
}

pub struct Resolver {
    receiver: Receiver<OrderAction>,
    contract: ResolverContract,
    chain_id: String,  
}

impl Resolver {
    pub fn new(receiver: Receiver<OrderAction>, chain_id: String, resolver_contract_address: String, provider: String) -> Self {
        let contract = ResolverContract::new(resolver_contract_address, "".to_string(), provider);
        Self {
            receiver,
            contract,
            chain_id,
        }
    }

    pub async fn run(&mut self) {
        tracing::info!(chain_id=?self.chain_id, "Resolver started");
        loop {
            match self.receiver.recv().await {
                Some(order_action) => {
                    self.process_order_action(order_action).await;
                }
                None => {
                    tracing::info!(chain_id=?self.chain_id, "Resolver channel closed, stopping");
                    break;
                }
            }
        }
    }

    async fn process_order_action(&self, order_action: OrderAction) {
        tracing::info!(
            chain_id=?self.chain_id,
            order_id=?order_action.order_id,
            action_type=?order_action.action_type,
            "Processing order action"
        );
        
        match order_action.action_type {
            ActionType::DeployEscrow => {
                self.deploy_escrow(&order_action).await;
            }
            ActionType::ReleaseFunds => {
                self.release_funds(&order_action).await;
            }
            ActionType::RefundFunds => {
                self.refund_funds(&order_action).await;
            }
            ActionType::NoOp => {
                tracing::info!(
                    chain_id=?self.chain_id,
                    order_id=?order_action.order_id,
                    "No action needed for order"
                );
            }
        }
    }

    async fn deploy_escrow(&self, order_action: &OrderAction) {
        tracing::info!(
            chain_id=?self.chain_id,
            order_id=?order_action.order_id,
            "Deploying escrow"
        );
        // Implementation for deploying escrow
    }

    async fn release_funds(&self, order_action: &OrderAction) {
        tracing::info!(
            chain_id=?self.chain_id,
            order_id=?order_action.order_id,
            "Releasing funds"
        );
        // Implementation for releasing funds
    }

    async fn refund_funds(&self, order_action: &OrderAction) {
        tracing::info!(
            chain_id=?self.chain_id,
            order_id=?order_action.order_id,
            "Refunding funds"
        );
        // Implementation for refunding funds
    }
}
