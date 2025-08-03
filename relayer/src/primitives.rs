use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderStatus {
    #[serde(rename = "unmatched")]
    Unmatched,
    #[serde(rename = "source_filled")]
    SourceFilled,
    #[serde(rename = "destination_filled")]
    DestinationFilled,
    #[serde(rename = "finality_confirmed")]
    FinalityConfirmed,
    #[serde(rename = "source_withdraw_pending")]
    SourceWithdrawPending,
    #[serde(rename = "destination_withdraw_pending")]
    DestinationWithdrawPending,
    #[serde(rename = "source_settled")]
    SourceSettled,
    #[serde(rename = "destination_settled")]
    DestinationSettled,
    #[serde(rename = "source_refunded")]
    SourceRefunded,
    #[serde(rename = "destination_refunded")]
    DestinationRefunded,
    #[serde(rename = "source_canceled")]
    SourceCanceled,
    #[serde(rename = "destination_canceled")]
    DestinationCanceled,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "fulfilled")]
    Fulfilled,
}

impl sqlx::Type<sqlx::Postgres> for OrderStatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("TEXT")
    }
}

impl sqlx::Decode<'_, sqlx::Postgres> for OrderStatus {
    fn decode(
        value: sqlx::postgres::PgValueRef<'_>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let string_value = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        match string_value.as_str() {
            "unmatched" => Ok(OrderStatus::Unmatched),
            "source_filled" => Ok(OrderStatus::SourceFilled),
            "destination_filled" => Ok(OrderStatus::DestinationFilled),
            "finality_confirmed" => Ok(OrderStatus::FinalityConfirmed),
            "source_settled" => Ok(OrderStatus::SourceSettled),
            "destination_settled" => Ok(OrderStatus::DestinationSettled),
            "source_withdraw_pending" => Ok(OrderStatus::SourceWithdrawPending),
            "destination_withdraw_pending" => Ok(OrderStatus::DestinationWithdrawPending),
            "expired" => Ok(OrderStatus::Expired),
            "source_refunded" => Ok(OrderStatus::SourceRefunded),
            "destination_refunded" => Ok(OrderStatus::DestinationRefunded),
            "source_canceled" => Ok(OrderStatus::SourceCanceled),
            "destination_canceled" => Ok(OrderStatus::DestinationCanceled),
            _ => Err(format!("Unknown order status: {}", string_value).into()),
        }
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Unmatched => write!(f, "unmatched"),
            OrderStatus::SourceFilled => write!(f, "source_filled"),
            OrderStatus::DestinationFilled => write!(f, "destination_filled"),
            OrderStatus::SourceSettled => write!(f, "source_settled"),
            OrderStatus::DestinationSettled => write!(f, "destination_settled"),
            OrderStatus::SourceWithdrawPending => write!(f, "source_withdraw_pending"),
            OrderStatus::DestinationWithdrawPending => write!(f, "destination_withdraw_pending"),
            OrderStatus::FinalityConfirmed => write!(f, "finality_confirmed"),
            OrderStatus::Expired => write!(f, "expired"),
            OrderStatus::SourceRefunded => write!(f, "source_refunded"),
            OrderStatus::DestinationRefunded => write!(f, "destination_refunded"),
            OrderStatus::SourceCanceled => write!(f, "source_canceled"),
            OrderStatus::DestinationCanceled => write!(f, "destination_canceled"),
            OrderStatus::Fulfilled => write!(f, "fulfilled"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum OrderType {
    #[serde(rename = "single_fill")]
    SingleFill,
    #[serde(rename = "multiple_fills")]
    MultipleFills,
}

impl sqlx::Type<sqlx::Postgres> for OrderType {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("TEXT")
    }
}

impl sqlx::Decode<'_, sqlx::Postgres> for OrderType {
    fn decode(
        value: sqlx::postgres::PgValueRef<'_>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let string_value = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        match string_value.as_str() {
            "single_fill" => Ok(OrderType::SingleFill),
            "multiple_fills" => Ok(OrderType::MultipleFills),
            _ => Err(format!("Unknown order type: {}", string_value).into()),
        }
    }
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::SingleFill => write!(f, "single_fill"),
            OrderType::MultipleFills => write!(f, "multiple_fills"),
        }
    }
}

/// Secret entry structure for storing secrets and their hashes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretEntry {
    pub index: u32,
    pub secret: Option<String>,
    pub secret_hash: String,
}

/// Order input data structure (user input)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderInput {
    /// Order salt
    pub salt: String,
    /// Source chain address of the maker asset
    pub maker_asset: String,
    /// Destination chain address of the taker asset
    pub taker_asset: String,
    /// Source chain address of the maker (wallet or contract address)
    pub maker: String,
    /// Destination chain address of the wallet or contract who will receive filled amount
    pub receiver: String,
    /// Order maker's token amount
    pub making_amount: BigDecimal,
    /// Order taker's token amount
    pub taking_amount: BigDecimal,
    /// Includes flags like: allow multiple fills, is partial fill allowed or not, price improvement, nonce, deadline etc.
    #[serde(default = "default_maker_traits")]
    pub maker_traits: String,
}

fn default_maker_traits() -> String {
    "0".to_string()
}

/// Signed order input for cross chain order submission (user input)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedOrderInput {
    /// Order Hash
    pub order_hash: String,
    /// Cross chain order data
    pub order: OrderInput,
    /// Source chain id
    pub src_chain_id: String,
    /// Destination chain id
    pub dst_chain_id: String,
    /// Signature of the cross chain order typed data (using signTypedData v4)
    pub signature: serde_json::Value,
    /// An interaction call data. ABI encoded a set of makerAssetSuffix, takerAssetSuffix, makingAmountGetter, takingAmountGetter, predicate, permit, preInteraction, postInteraction
    pub extension: serde_json::Value,
    /// Order type (single fill or multiple fills)
    pub order_type: OrderType,
    /// Secret entries containing index, secret, and secret_hash
    pub secrets: Vec<SecretEntry>,
    /// Deadline by which the order must be filled (Unix timestamp in milliseconds)
    pub deadline: u64,
    /// Taker address
    pub taker: String,
    /// Timelock for the order
    pub timelock: String,
    /// Taker traits
    pub taker_traits: String,
    /// Args
    pub args: serde_json::Value,
    /// Source chain deploy immutables data
    pub src_deploy_immutables: Option<serde_json::Value>,
    /// Destination chain deploy immutables data
    pub dst_deploy_immutables: Option<serde_json::Value>,
    /// Source chain withdraw immutables data
    pub src_withdraw_immutables: Option<serde_json::Value>,
    /// Destination chain withdraw immutables data
    pub dst_withdraw_immutables: Option<serde_json::Value>,
}

/// Database model for cross chain orders
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CrossChainOrder {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub order_hash: String,
    pub src_chain_id: String,
    pub dst_chain_id: String,
    pub maker: String,
    pub receiver: String,
    pub taker: String,
    pub timelock: String,
    pub maker_asset: String,
    pub taker_asset: String,
    pub making_amount: BigDecimal,
    pub taking_amount: BigDecimal,
    pub salt: String,
    pub maker_traits: String,
    pub taker_traits: String,
    pub args: serde_json::Value,
    pub signature: serde_json::Value,
    pub extension: serde_json::Value,
    pub order_type: OrderType,
    pub secrets: serde_json::Value,
    pub status: OrderStatus,
    pub deadline: i64,
    pub auction_start_date: Option<String>,
    pub auction_end_date: Option<String>,
    pub src_escrow_address: Option<String>,
    pub dst_escrow_address: Option<String>,
    pub src_tx_hash: Option<String>,
    pub dst_tx_hash: Option<String>,
    pub filled_maker_amount: BigDecimal,
    pub filled_taker_amount: BigDecimal,
    pub src_deploy_immutables: Option<serde_json::Value>,
    pub dst_deploy_immutables: Option<serde_json::Value>,
    pub src_withdraw_immutables: Option<serde_json::Value>,
    pub dst_withdraw_immutables: Option<serde_json::Value>,
    pub src_event: Option<serde_json::Value>,
    pub dest_event: Option<serde_json::Value>,
    pub src_withdraw: Option<serde_json::Value>,
    pub dst_withdraw: Option<serde_json::Value>,
}

/// Secret input for order fill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretInput {
    /// A secret for the fill hashlock
    pub secret: String,
    /// Order hash
    pub order_hash: String
}


/// Active order output for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveOrderOutput {
    pub order_hash: String, 
    pub signature: serde_json::Value,
    pub deadline: u64,
    pub auction_start_date: Option<String>,
    pub auction_end_date: Option<String>,
    pub remaining_maker_amount: String,
    pub extension: serde_json::Value,
    pub src_chain_id: String,
    pub dst_chain_id: String,
    pub order: OrderInput,
    pub taker: String,
    pub timelock: String,
    pub taker_traits: String,
    pub args: serde_json::Value,
    pub order_type: OrderType,
    pub secrets: Vec<SecretEntry>,
    pub status: OrderStatus,
    pub src_deploy_immutables: Option<serde_json::Value>,
    pub dst_deploy_immutables: Option<serde_json::Value>,
    pub src_withdraw_immutables: Option<serde_json::Value>,
    pub dst_withdraw_immutables: Option<serde_json::Value>,
    pub src_event: Option<serde_json::Value>,
    pub dest_event: Option<serde_json::Value>,
    pub src_withdraw: Option<serde_json::Value>,
    pub dst_withdraw: Option<serde_json::Value>,
}

/// Meta information for paginated responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub total_items: u64,
    pub items_per_page: u64,
    pub total_pages: u64,
    pub current_page: u64,
}

/// Paginated active orders response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetActiveOrdersOutput {
    pub meta: Meta,
    pub items: Vec<ActiveOrderOutput>,
}

/// Secret response for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretResponse {
    pub secret: Option<String>,
    pub order_hash: String,
}

/// Request to update a specific field for an order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderFieldRequest {
    /// Order hash
    pub order_hash: String,
    /// Field name to update (must be one of the valid JSONB fields)
    pub field_name: String,
    /// JSON value to set for the field
    pub value: serde_json::Value,
}
