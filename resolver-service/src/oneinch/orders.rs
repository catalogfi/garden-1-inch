use std::time::Duration;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use reqwest::Client;
use tracing;

/// Orders API client for Garden 1-Inch Relayer
pub struct OrdersClient {
    client: Client,
    base_url: String,
}

impl OrdersClient {
    /// Create a new orders client
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::builder().timeout(Duration::from_secs(10)).build().unwrap(),
            base_url,
        }
    }

    /// Get active orders with pagination
    pub async fn get_active_orders(&self, params: ActiveOrdersParams) -> Result<GetActiveOrdersOutput> {
        let url = format!("{}/orders/active", self.base_url);
        let response = self.client
            .get(&url)
            .query(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let response_text = response.text().await?;
            tracing::debug!("Raw response: {}", response_text);
            
            let result: ApiResponse<GetActiveOrdersOutput> = serde_json::from_str(&response_text)?;
            match result.status.as_str() {
                "Ok" => Ok(result.result.unwrap()),
                "Error" => Err(anyhow::anyhow!("API Error: {}", result.error.unwrap_or_default())),
                _ => Err(anyhow::anyhow!("Unknown API status: {}", result.status)),
            }
        } else {
            Err(anyhow::anyhow!("Failed to get active orders: {}", response.status()))
        }
    }

    /// Get order by hash
    pub async fn get_order_by_hash(&self, order_hash: &str) -> Result<OrderDetail> {
        let url = format!("{}/orders/{}", self.base_url, order_hash);
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if response.status().is_success() {
            let result: ApiResponse<OrderDetail> = response.json().await?;
            match result.status.as_str() {
                "Ok" => Ok(result.result.unwrap()),
                "Error" => Err(anyhow::anyhow!("API Error: {}", result.error.unwrap_or_default())),
                _ => Err(anyhow::anyhow!("Unknown API status: {}", result.status)),
            }
        } else {
            Err(anyhow::anyhow!("Failed to get order by hash: {}", response.status()))
        }
    }

    /// Get orders by chain ID
    pub async fn get_orders_by_chain(&self, chain_id: u64) -> Result<Vec<OrderDetail>> {
        let url = format!("{}/orders/chain/{}", self.base_url, chain_id);
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if response.status().is_success() {
            let result: ApiResponse<Vec<OrderDetail>> = response.json().await?;
            match result.status.as_str() {
                "Ok" => Ok(result.result.unwrap()),
                "Error" => Err(anyhow::anyhow!("API Error: {}", result.error.unwrap_or_default())),
                _ => Err(anyhow::anyhow!("Unknown API status: {}", result.status)),
            }
        } else {
            Err(anyhow::anyhow!("Failed to get orders by chain: {}", response.status()))
        }
    }

    /// Get secrets for an order
    pub async fn get_secrets(&self, order_hash: &str) -> Result<SecretResponse> {
        let url = format!("{}/orders/secret/{}", self.base_url, order_hash);
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if response.status().is_success() {
            let result: ApiResponse<SecretResponse> = response.json().await?;
            match result.status.as_str() {
                "Ok" => Ok(result.result.unwrap()),
                "Error" => Err(anyhow::anyhow!("API Error: {}", result.error.unwrap_or_default())),
                _ => Err(anyhow::anyhow!("Unknown API status: {}", result.status)),
            }
        } else {
            Err(anyhow::anyhow!("Failed to get secrets: {}", response.status()))
        }
    }
}

/// Generic API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub result: Option<T>,
    pub error: Option<String>,
}

/// Parameters for getting active orders
#[derive(Debug, Serialize)]
pub struct ActiveOrdersParams {
    /// Page number (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    /// Number of items per page (default: 100, max: 500)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

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
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum OrderType {
    #[serde(rename = "single_fill")]
    SingleFill,
    #[serde(rename = "multiple_fills")]
    MultipleFills,
}

/// Signature structure for order signatures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub r: String,
    pub vs: String,
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

/// Active order output for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveOrderOutput {
    pub order_hash: String,
    pub signature: Signature,
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
    pub args: String,
    pub order_type: OrderType,
    pub secrets: Vec<SecretEntry>
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

/// Database model for cross chain orders
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub src_deploy_immutables: serde_json::Value,
    pub dst_deploy_immutables: serde_json::Value,
    pub src_withdraw_immutables: serde_json::Value,
    pub dst_withdraw_immutables: serde_json::Value,
}

/// Detailed order information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderDetail {
    pub created_at: String,
    pub updated_at: String,
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
    pub secrets: Vec<SecretEntry>,
    pub status: OrderStatus,
    pub deadline: u64,
    pub auction_start_date: Option<String>,
    pub auction_end_date: Option<String>,
    pub src_escrow_address: Option<String>,
    pub dst_escrow_address: Option<String>,
    pub src_tx_hash: Option<String>,
    pub dst_tx_hash: Option<String>,
    pub filled_maker_amount: BigDecimal,
    pub filled_taker_amount: BigDecimal,
    pub src_deploy_immutables: serde_json::Value,
    pub dst_deploy_immutables: serde_json::Value,
    pub src_withdraw_immutables: serde_json::Value,
    pub dst_withdraw_immutables: serde_json::Value,
}

/// Secret response for getting order secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretResponse {
    pub secret: Option<String>,
    pub order_hash: String,
}

impl ActiveOrdersParams {
    /// Create a new active orders params
    pub fn new() -> Self {
        Self {
            page: None,
            limit: None,
        }
    }

    /// Set the page
    pub fn with_page(mut self, page: u64) -> Self {
        self.page = Some(page);
        self
    }

    /// Set the limit
    pub fn with_limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }
}
