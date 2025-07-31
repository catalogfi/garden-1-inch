use std::sync::Arc;

use alloy::{network::Ethereum, providers::RootProvider};
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};

pub type StarknetClient = JsonRpcClient<HttpTransport>;
pub type EthereumClient = RootProvider<Ethereum>;

#[derive(Debug, Clone)]
pub enum ChainType {
    Ethereum,
    Starknet,
}

#[derive(Clone, Debug)]
pub enum ClientType {
    Ethereum(Arc<EthereumClient>),
    Starknet(Arc<StarknetClient>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum WatcherEventType {
    SourceEscrowCreated,
    SourceEscrowUpdated,
    SourceEscrowClosed,
    DestinationEscrowCreated,
    DestinationEscrowUpdated,
    DestinationEscrowClosed,
}

impl ChainType {
    pub fn is_ethereum(&self) -> bool {
        matches!(self, ChainType::Ethereum)
    }

    pub fn is_starknet(&self) -> bool {
        matches!(self, ChainType::Starknet)
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            ChainType::Ethereum => "Ethereum",
            ChainType::Starknet => "Starknet",
        }
    }
}

pub enum OrderStatus {
    Unmatched,
    SourceFilled,
    DestinationFilled,
    SourceWithdrawPending,
    DestinationWithdrawPending,
    SourceSettled,
    DestinationSettled,
    Expired,
    Refunded,
}

impl OrderStatus {
    pub fn to_string(&self) -> &str {
        match self {
            OrderStatus::Unmatched => "unmatched",
            OrderStatus::SourceFilled => "source_filled",
            OrderStatus::DestinationFilled => "destination_filled",
            OrderStatus::SourceWithdrawPending => "source_withdraw_pending",
            OrderStatus::DestinationWithdrawPending => "destination_withdraw_pending",
            OrderStatus::SourceSettled => "source_settled",
            OrderStatus::DestinationSettled => "destination_settled",
            OrderStatus::Expired => "expired",
            OrderStatus::Refunded => "refunded",
        }
    }
}
