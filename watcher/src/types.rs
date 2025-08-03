use alloy::{network::Ethereum, providers::RootProvider};
use serde::{Deserialize, Serialize};
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};

pub type StarknetClient = JsonRpcClient<HttpTransport>;
pub type EthereumClient = RootProvider<Ethereum>;

/// Secret entry structure for storing secrets and their hashes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretEntry {
    pub index: u32,
    pub secret: Option<String>,
    pub secret_hash: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ChainType {
    Ethereum(String),
    Starknet(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum WatcherEventType {
    SrcEscrowCreatedEvent,
    DstEscrowCreatedEvent,
    SourceWithdraw,
    DestinationWithdraw,
    SourceRescue,
    DestinationRescue,
}

impl ChainType {
    pub fn name(&self) -> &str {
        match self {
            ChainType::Ethereum(name) => name,
            ChainType::Starknet(name) => name,
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
    FulFilled,
}

impl OrderStatus {
    pub fn to_string(&self) -> &str {
        match self {
            OrderStatus::Unmatched => "unmatched",
            OrderStatus::SourceFilled => "source_filled",
            OrderStatus::DestinationFilled => "destination_filled",
            OrderStatus::FinalityConfirmed => "finality_confirmed",
            OrderStatus::SourceWithdrawPending => "source_withdraw_pending",
            OrderStatus::DestinationWithdrawPending => "destination_withdraw_pending",
            OrderStatus::SourceSettled => "source_settled",
            OrderStatus::DestinationSettled => "destination_settled",
            OrderStatus::SourceRefunded => "source_refunded",
            OrderStatus::DestinationRefunded => "destination_refunded",
            OrderStatus::SourceCanceled => "source_canceled",
            OrderStatus::DestinationCanceled => "destination_canceled",
            OrderStatus::Expired => "expired",
            OrderStatus::FulFilled => "fulfilled",
        }
    }
}
