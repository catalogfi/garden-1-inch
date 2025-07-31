use std::sync::mpsc::Receiver;

use crate::order_mapper::{OrderAction, ActionType};

pub struct ResolverContract {
    address: String,
    abi: String,
}

impl ResolverContract {
    pub fn new() -> Self {
        Self {
            address: String::new(),
            abi: String::new(),
        }
    }
}

pub struct Resolver {
    receiver: Receiver<OrderAction>,
    contract: ResolverContract
}

impl Resolver {
    pub fn new(receiver: Receiver<OrderAction>) -> Self {
        Self {
            receiver,
            contract: ResolverContract::new(),
        }
    }

    pub async fn run(&self) {
        loop {
            match self.receiver.recv() {
                Ok(order_action) => {
                    self.process_order_action(order_action).await;
                }
                Err(e) => {
                    println!("Error receiving order action: {}", e);
                    continue;
                }
            }
        }
    }

    async fn process_order_action(&self, order_action: OrderAction) {
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
                tracing::info!("No action needed for order: {}", order_action.order_id);
            }
        }
    }

    async fn deploy_escrow(&self, order_action: &OrderAction) {
        tracing::info!("Deploying escrow for order: {}", order_action.order_id);
        tracing::info!("Order data: {:?}", order_action.order);
        // Implementation for deploying escrow
    }

    async fn release_funds(&self, order_action: &OrderAction) {
        tracing::info!("Releasing funds for order: {}", order_action.order_id);
        tracing::info!("Order data: {:?}", order_action.order);
        // Implementation for releasing funds
    }

    async fn refund_funds(&self, order_action: &OrderAction) {
        tracing::info!("Refunding funds for order: {}", order_action.order_id);
        tracing::info!("Order data: {:?}", order_action.order);
        // Implementation for refunding funds
    }
}
