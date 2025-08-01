use serde::{Deserialize, Serialize};
use anyhow::Result;
use reqwest::Client;

/// Orders API client for Garden 1-Inch Relayer
pub struct OrdersClient {
    client: Client,
    base_url: String,
}

impl OrdersClient {
    /// Create a new orders client
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
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
            let result: ApiResponse<GetActiveOrdersOutput> = response.json().await?;
            match result.status.as_str() {
                "ok" => Ok(result.result.unwrap()),
                "error" => Err(anyhow::anyhow!("API Error: {}", result.error.unwrap_or_default())),
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
                "ok" => Ok(result.result.unwrap()),
                "error" => Err(anyhow::anyhow!("API Error: {}", result.error.unwrap_or_default())),
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
                "ok" => Ok(result.result.unwrap()),
                "error" => Err(anyhow::anyhow!("API Error: {}", result.error.unwrap_or_default())),
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
                "ok" => Ok(result.result.unwrap()),
                "error" => Err(anyhow::anyhow!("API Error: {}", result.error.unwrap_or_default())),
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

/// Order structure from the API
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub salt: String,
    pub maker_asset: String,
    pub taker_asset: String,
    pub maker: String,
    pub receiver: String,
    pub making_amount: String,
    pub taking_amount: String,
    pub maker_traits: String,
}

/// Secret entry structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecretEntry {
    pub index: u64,
    pub secret: Option<String>,
    pub secret_hash: String,
}

/// Order type enum
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    SingleFill,
    MultipleFills,
}

/// Order status enum
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
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

/// Active order output structure
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActiveOrder {
    pub order_hash: String,
    pub signature: String,
    pub deadline: u64,
    pub auction_start_date: Option<String>,
    pub auction_end_date: Option<String>,
    pub remaining_maker_amount: String,
    pub extension: String,
    pub src_chain_id: u64,
    pub dst_chain_id: u64,
    pub order: Order,
    pub order_type: OrderType,
    pub secrets: Vec<SecretEntry>,
}

/// Meta information for pagination
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub total_items: u64,
    pub items_per_page: u64,
    pub total_pages: u64,
    pub current_page: u64,
}

/// Get active orders output
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetActiveOrdersOutput {
    pub meta: Meta,
    pub items: Vec<ActiveOrder>,
}

/// Detailed order information
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetail {
    pub created_at: String,
    pub updated_at: String,
    pub order_hash: String,
    pub src_chain_id: u64,
    pub dst_chain_id: u64,
    pub maker: String,
    pub receiver: String,
    pub maker_asset: String,
    pub taker_asset: String,
    pub making_amount: String,
    pub taking_amount: String,
    pub salt: String,
    pub maker_traits: String,
    pub signature: String,
    pub extension: String,
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
    pub filled_maker_amount: String,
    pub filled_taker_amount: String,
}

/// Secret response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SecretResponse {
    pub secret: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_orders_params_creation() {
        let params = ActiveOrdersParams::new()
            .with_page(1)
            .with_limit(100);

        assert_eq!(params.page, Some(1));
        assert_eq!(params.limit, Some(100));
    }

    #[test]
    fn test_order_creation() {
        let order = Order {
            salt: "0x1234567890abcdef".to_string(),
            maker_asset: "0x1234567890123456789012345678901234567890".to_string(),
            taker_asset: "0x0987654321098765432109876543210987654321".to_string(),
            maker: "0x1111111111111111111111111111111111111111".to_string(),
            receiver: "0x2222222222222222222222222222222222222222".to_string(),
            making_amount: "1000000000000000000".to_string(),
            taking_amount: "2000000000000000000".to_string(),
            maker_traits: "0".to_string(),
        };

        assert_eq!(order.salt, "0x1234567890abcdef");
        assert_eq!(order.maker_asset, "0x1234567890123456789012345678901234567890");
        assert_eq!(order.taker_asset, "0x0987654321098765432109876543210987654321");
        assert_eq!(order.maker, "0x1111111111111111111111111111111111111111");
        assert_eq!(order.receiver, "0x2222222222222222222222222222222222222222");
        assert_eq!(order.making_amount, "1000000000000000000");
        assert_eq!(order.taking_amount, "2000000000000000000");
        assert_eq!(order.maker_traits, "0");
    }

    #[test]
    fn test_secret_entry_creation() {
        let secret_entry = SecretEntry {
            index: 0,
            secret: None,
            secret_hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890".to_string(),
        };

        assert_eq!(secret_entry.index, 0);
        assert_eq!(secret_entry.secret, None);
        assert_eq!(secret_entry.secret_hash, "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890");
    }

    #[test]
    fn test_order_type_enum() {
        assert_eq!(OrderType::SingleFill, OrderType::SingleFill);
        assert_eq!(OrderType::MultipleFills, OrderType::MultipleFills);
        assert_ne!(OrderType::SingleFill, OrderType::MultipleFills);
    }

    #[test]
    fn test_order_status_enum() {
        assert_eq!(OrderStatus::Unmatched, OrderStatus::Unmatched);
        assert_eq!(OrderStatus::SourceFilled, OrderStatus::SourceFilled);
        assert_ne!(OrderStatus::Unmatched, OrderStatus::SourceFilled);
    }
}
