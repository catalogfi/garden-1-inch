use alloy::{network::Ethereum, providers::RootProvider};
use serde::{Deserialize, Serialize};
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};

pub type StarknetClient = JsonRpcClient<HttpTransport>;
pub type EthereumClient = RootProvider<Ethereum>;

#[derive(Debug, Clone)]
pub enum ChainType {
    Ethereum,
    Starknet,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderStatus {
    Unmatched,
    SourceFilled,
    DestinationFilled,
    FinalityConfirmed,
    SourceWithdrawPending,
    DestinationWithdrawPending,
    SourceSettled,
    DestinationSettled,
    SourceRefunded,
    DestinationRefunded,
    SourceCanceled,
    DestinationCanceled,
    Expired,
}

impl OrderStatus {
    pub fn to_string(&self) -> &str {
        match self {
            OrderStatus::Unmatched => "Unmatched",
            OrderStatus::SourceFilled => "SourceFilled",
            OrderStatus::DestinationFilled => "DestinationFilled",
            OrderStatus::FinalityConfirmed => "FinalityConfirmed",
            OrderStatus::SourceWithdrawPending => "SourceWithdrawPending",
            OrderStatus::DestinationWithdrawPending => "DestinationWithdrawPending",
            OrderStatus::SourceSettled => "SourceSettled",
            OrderStatus::DestinationSettled => "DestinationSettled",
            OrderStatus::SourceRefunded => "SourceRefunded",
            OrderStatus::DestinationRefunded => "DestinationRefunded",
            OrderStatus::SourceCanceled => "SourceCanceled",
            OrderStatus::DestinationCanceled => "DestinationCanceled",
            OrderStatus::Expired => "Expired",
        }
    }
}
