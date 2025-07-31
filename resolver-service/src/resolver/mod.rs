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

    pub fn run(&self) {
        loop {
            match self.receiver.recv() {
                Ok(order_action) => {
                    self.process_order_action(order_action);
                }
                Err(e) => {
                    println!("Error receiving order action: {}", e);
                    continue;
                }
            }
        }
    }

    fn process_order_action(&self, order_action: OrderAction) {
        match order_action.action_type {
            ActionType::DeployEscrow => {
                self.deploy_escrow(&order_action);
            }
            ActionType::ReleaseFunds => {
                self.release_funds(&order_action);
            }
            ActionType::RefundFunds => {
                self.refund_funds(&order_action);
            }
        }
    }

    fn deploy_escrow(&self, order_action: &OrderAction) {
        println!("Deploying escrow for order: {}", order_action.order_id);
        println!("Order data: {:?}", order_action.order);
        // Implementation for deploying escrow
    }

    fn release_funds(&self, order_action: &OrderAction) {
        println!("Releasing funds for order: {}", order_action.order_id);
        println!("Order data: {:?}", order_action.order);
        // Implementation for releasing funds
    }

    fn refund_funds(&self, order_action: &OrderAction) {
        println!("Refunding funds for order: {}", order_action.order_id);
        println!("Order data: {:?}", order_action.order);
        // Implementation for refunding funds
    }
}
