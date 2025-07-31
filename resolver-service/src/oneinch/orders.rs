use serde::{Deserialize, Serialize};
use anyhow::Result;
use reqwest::Client;

/// Orders API client for 1inch Fusion+
pub struct OrdersClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl OrdersClient {
    /// Create a new orders client
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }

    /// Get cross chain swap active orders
    pub async fn get_active_orders(&self, params: ActiveOrdersParams) -> Result<GetActiveOrdersOutput> {
        let url = format!("{}/orders/v1.0/order/active", self.base_url);
        let response = self.client
            .get(&url)
            .query(&params)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            // println!("response: {:?}", response.text().await?);
            let orders: GetActiveOrdersOutput = response.json().await.unwrap();
            Ok(orders)
        } else {
            Err(anyhow::anyhow!("Failed to get active orders: {}", response.status()))
        }
    }

    /// Get actual escrow factory contract address
    pub async fn get_settlement_contract(&self, chain_id: Option<u64>) -> Result<EscrowFactory> {
        let url = format!("{}/orders/v1.0/order/escrow", self.base_url);
        
        let mut request = self.client.get(&url);
        if let Some(chain_id) = chain_id {
            request = request.query(&[("chainId", chain_id.to_string())]);
        }
        
        let response = request.header("Authorization", format!("Bearer {}", self.api_key)).send().await?;

        if response.status().is_success() {
            let escrow: EscrowFactory = response.json().await?;
            Ok(escrow)
        } else {
            Err(anyhow::anyhow!("Failed to get settlement contract: {}", response.status()))
        }
    }

    /// Get orders by maker address
    pub async fn get_orders_by_maker(&self, address: String, params: OrdersByMakerParams) -> Result<GetOrderByMakerOutput> {
        let url = format!("{}/orders/v1.0/order/maker/{}", self.base_url, address);
        
        let response = self.client
            .get(&url)
            .query(&params)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let orders: GetOrderByMakerOutput = response.json().await?;
            Ok(orders)
        } else {
            Err(anyhow::anyhow!("Failed to get orders by maker: {}", response.status()))
        }
    }

    /// Get all data to perform withdrawal and cancellation
    pub async fn get_published_secrets(&self, order_hash: String) -> Result<ResolverDataOutput> {
        let url = format!("{}/orders/v1.0/order/secrets/{}", self.base_url, order_hash);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let secrets: ResolverDataOutput = response.json().await?;
            Ok(secrets)
        } else {
            Err(anyhow::anyhow!("Failed to get published secrets: {}", response.status()))
        }
    }

    /// Get idx of each secret that is ready for submission for specific order
    pub async fn get_ready_to_accept_secret_fills(&self, order_hash: String) -> Result<ReadyToAcceptSecretFills> {
        let url = format!("{}/orders/v1.0/order/ready-to-accept-secret-fills/{}", self.base_url, order_hash);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let fills: ReadyToAcceptSecretFills = response.json().await?;
            Ok(fills)
        } else {
            Err(anyhow::anyhow!("Failed to get ready to accept secret fills: {}", response.status()))
        }
    }

    /// Get idx of each secret that is ready for submission for all orders
    pub async fn get_ready_to_accept_secret_fills_for_all_orders(&self) -> Result<ReadyToAcceptSecretFillsForAllOrders> {
        let url = format!("{}/orders/v1.0/order/ready-to-accept-secret-fills", self.base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let fills: ReadyToAcceptSecretFillsForAllOrders = response.json().await?;
            Ok(fills)
        } else {
            Err(anyhow::anyhow!("Failed to get ready to accept secret fills for all orders: {}", response.status()))
        }
    }

    /// Get all data to perform a cancellation or withdrawal on public periods
    pub async fn get_events_ready_for_public_action(&self) -> Result<ReadyToExecutePublicActionsOutput> {
        let url = format!("{}/orders/v1.0/order/ready-to-execute-public-actions", self.base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let actions: ReadyToExecutePublicActionsOutput = response.json().await?;
            Ok(actions)
        } else {
            Err(anyhow::anyhow!("Failed to get events ready for public action: {}", response.status()))
        }
    }

    /// Get order by hash
    pub async fn get_order_by_order_hash(&self, order_hash: String) -> Result<GetOrderFillsByHashOutput> {
        let url = format!("{}/orders/v1.0/order/status/{}", self.base_url, order_hash);
        println!("url: {}", url);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        if response.status().is_success() {
            let order: GetOrderFillsByHashOutput = response.json().await?;
            Ok(order)
        } else {
            Err(anyhow::anyhow!("Failed to get order by hash: {}", response.status()))
        }
    }

    /// Get orders by hashes
    pub async fn get_orders_by_order_hashes(&self, order_hashes: Vec<String>) -> Result<GetOrderFillsByHashOutput> {
        let url = format!("{}/orders/v1.0/order/status", self.base_url);
        
        let body = OrdersByHashesInput { order_hashes };
        
        let response = self.client
            .post(&url)
            .json(&body)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let orders: GetOrderFillsByHashOutput = response.json().await?;
            Ok(orders)
        } else {
            Err(anyhow::anyhow!("Failed to get orders by hashes: {}", response.status()))
        }
    }
}

/// Parameters for getting active orders
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveOrdersParams {
    /// Pagination step, default: 1 (page = offset / limit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    /// Number of active orders to receive (default: 100, max: 500)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Source chain of cross chain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_chain: Option<u64>,
    /// Destination chain of cross chain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_chain: Option<u64>,
}

/// Parameters for getting orders by maker
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrdersByMakerParams {
    /// Pagination step, default: 1 (page = offset / limit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    /// Number of active orders to receive (default: 100, max: 500)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// timestampFrom in milliseconds for interval [timestampFrom, timestampTo)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_from: Option<u64>,
    /// timestampTo in milliseconds for interval [timestampFrom, timestampTo)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_to: Option<u64>,
    /// Find history by the given source token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_token: Option<String>,
    /// Find history by the given destination token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_token: Option<String>,
    /// Find history items by source or destination token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_token: Option<String>,
    /// Destination chain of cross chain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_chain_id: Option<u64>,
    /// Source chain of cross chain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_chain_id: Option<u64>,
    /// chainId for looking by dstChainId == chainId OR srcChainId == chainId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<u64>,
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

/// Cross chain order DTO
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CrossChainOrderDto {
    /// Some unique value. It is necessary to be able to create cross chain orders with the same parameters (so that they have a different hash), Lowest 160 bits of the order salt must be equal to the lowest 160 bits of the extension hash
    pub salt: String,
    /// Address of the account creating the order (maker) in src chain.
    pub maker: String,
    /// Address of the account receiving the assets (receiver), if different from maker in dst chain.
    pub receiver: String,
    /// Identifier of the asset being offered by the maker in src chain.
    pub maker_asset: String,
    /// Identifier of the asset being requested by the maker in exchange in dst chain.
    pub taker_asset: String,
    /// Amount of the makerAsset being offered by the maker in src chain.
    pub making_amount: String,
    /// Amount of the takerAsset being requested by the maker in dst chain.
    pub taking_amount: String,
    /// Includes some flags like, allow multiple fills, is partial fill allowed or not, price improvement, nonce, deadline etc.
    pub maker_traits: String,
}

/// Active orders output
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActiveOrdersOutput {
    /// Unique identifier
    pub id: String,
    /// Unique identifier of the order.
    pub order_hash: String,
    /// Identifier of the quote associated with this order.
    pub quote_id: String,
    /// Identifier of the chain where the maker asset is located.
    pub src_chain_id: u64,
    /// Identifier of the chain where the taker asset is located.
    pub dst_chain_id: u64,
    /// Address of the account creating the order (maker) in src chain.
    pub maker: String,
    /// Address of the account receiving the assets (receiver), if different from maker in dst chain.
    pub receiver: String,
    /// Identifier of the asset being offered by the maker in src chain.
    pub maker_asset: String,
    /// Identifier of the asset being requested by the maker in exchange in dst chain.
    pub taker_asset: String,
    /// Amount of the makerAsset being offered by the maker in src chain.
    pub making_amount: String,
    /// Amount of the takerAsset being requested by the maker in dst chain.
    pub taking_amount: String,
    /// Some unique value. It is necessary to be able to create cross chain orders with the same parameters
    pub salt: String,
    /// Includes some flags like, allow multiple fills, is partial fill allowed or not, price improvement, nonce, deadline etc.
    pub maker_traits: String,
    /// Signature of the order.
    pub signature: String,
    /// An interaction call data. ABI encoded set of makerAssetSuffix, takerAssetSuffix, makingAmountGetter, takingAmountGetter, predicate, permit, preInteraction, postInteraction
    pub extension: String,
    /// Array of secret hashes.
    pub secret_hashes: Vec<Vec<String>>,
    /// Array of secrets.
    pub secrets: Vec<String>,
    /// Order status
    pub status: String,
    /// Deadline by which the order must be filled.
    pub deadline: u64,
    /// Start date of the auction for this order.
    pub auction_start_date: Option<u64>,
    /// End date of the auction for this order.
    pub auction_end_date: Option<u64>,
    /// Source escrow address
    pub src_escrow_address: Option<String>,
    /// Destination escrow address
    pub dst_escrow_address: Option<String>,
    /// Source transaction hash
    pub src_tx_hash: Option<String>,
    /// Destination transaction hash
    pub dst_tx_hash: Option<String>,
    /// Amount of the makerAsset filled in src chain.
    pub filled_maker_amount: String,
    /// Amount of the takerAsset filled in dst chain.
    pub filled_taker_amount: String,
    /// Creation timestamp
    pub created_at: String,
    /// Last update timestamp
    pub updated_at: String,
}

/// Get active orders output
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetActiveOrdersOutput {
    pub meta: Meta,
    pub items: Vec<ActiveOrdersOutput>,
}

/// Escrow factory
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EscrowFactory {
    /// actual escrow factory contract address
    pub address: String,
}

/// Get order by maker output
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderByMakerOutput {
    pub meta: Meta,
    pub items: Vec<ActiveOrdersOutput>,
}

/// Immutables
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Immutables {
    /// Order's hash 32 bytes hex sting
    pub order_hash: String,
    /// keccak256(secret(idx))
    pub hashlock: String,
    /// Maker's address which will receive tokens
    pub maker: String,
    /// Escrow creation initiator address
    pub taker: String,
    /// Token to receive on specific chain
    pub token: String,
    /// Amount of token to receive
    pub amount: String,
    /// Security deposit in chain's native currency
    pub safety_deposit: String,
    /// Encoded timelocks. To decode use: https://github.com/1inch/cross-chain-sdk/blob/master/src/cross-chain-order/time-locks/time-locks.ts
    pub timelocks: String,
}

/// Public secret
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicSecret {
    /// Sequence number of secrets
    pub idx: u64,
    /// Public secret to perform a withdrawal
    pub secret: String,
    /// Source chain immutables to provide in withdraw/publicWithdraw/cancel/publicCancel functions
    pub src_immutables: Immutables,
    /// Destination chain immutables to provide in withdraw/publicWithdraw/cancel functions
    pub dst_immutables: Immutables,
}

/// Resolver data output
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolverDataOutput {
    /// Type of the order: enabled or disabled partial fills
    pub order_type: String,
    /// The data required for order withdraw and cancel
    pub secrets: Vec<PublicSecret>,
    /// keccak256(secret(idx))[]
    pub secret_hashes: Vec<Vec<String>>,
}

/// Ready to accept secret fill
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadyToAcceptSecretFill {
    /// Sequence number of secrets for submission
    pub idx: u64,
    /// Transaction hash where the source chain escrow was deployed
    pub src_escrow_deploy_tx_hash: String,
    /// Transaction hash where the destination chain escrow was deployed
    pub dst_escrow_deploy_tx_hash: String,
}

/// Ready to accept secret fills
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadyToAcceptSecretFills {
    /// Fills that are ready to accept secrets from the client
    pub fills: Vec<ReadyToAcceptSecretFill>,
}

/// Ready to accept secret fills for order
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadyToAcceptSecretFillsForOrder {
    /// Order hash
    pub order_hash: String,
    /// Maker address
    pub maker_address: String,
    /// Fills that are ready to accept secrets from the client
    pub fills: Vec<ReadyToAcceptSecretFill>,
}

/// Ready to accept secret fills for all orders
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadyToAcceptSecretFillsForAllOrders {
    /// Fills that are ready to accept secrets from the client for all orders
    pub orders: Vec<ReadyToAcceptSecretFillsForOrder>,
}

/// Ready to execute public action
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadyToExecutePublicAction {
    /// Action type
    pub action: String,
    /// Chain's immutables to provide for execution
    pub immutables: Immutables,
    /// Execute action on this chain
    pub chain_id: u64,
    /// Escrow's address to perform public action
    pub escrow: String,
    /// Presented only for withdraw action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

/// Ready to execute public actions output
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadyToExecutePublicActionsOutput {
    /// Actions allowed to be performed on public timelock periods
    pub actions: Vec<ReadyToExecutePublicAction>,
}

/// Limit order V4 struct output
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrderV4StructOutput {
    pub salt: String,
    /// Maker address
    pub maker: String,
    /// Receiver address
    pub receiver: String,
    /// Maker asset address
    pub maker_asset: String,
    /// Taker asset address
    pub taker_asset: String,
    /// Amount of the maker asset
    pub making_amount: String,
    /// Amount of the taker asset
    pub taking_amount: String,
    pub maker_traits: String,
}

/// Auction point output
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionPointOutput {
    /// The delay in seconds from the previous point or auction start time
    pub delay: u64,
    /// The rate bump from the order min taker amount
    pub coefficient: u64,
}

/// Escrow event data output
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EscrowEventDataOutput {
    /// Transaction hash
    pub transaction_hash: String,
    /// The address of the escrow where the action happened
    pub escrow: String,
    /// Side of the escrow event SRC or DST
    pub side: String,
    /// Action of the escrow event
    pub action: String,
    /// Unix timestamp in milliseconds
    pub block_timestamp: u64,
}

/// Fill output DTO
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillOutputDto {
    /// Fill status
    pub status: String,
    /// Transaction hash
    pub tx_hash: String,
    /// Amount of the makerAsset filled in src chain.
    pub filled_maker_amount: String,
    /// Amount of the takerAsset filled in dst chain.
    pub filled_auction_taker_amount: String,
    pub escrow_events: Vec<EscrowEventDataOutput>,
}

/// Get order fills by hash output
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderFillsByHashOutput {
    /// Order hash
    pub order_hash: String,
    /// Order status
    pub status: String,
    /// Order validation status
    pub validation: String,
    /// Order data
    pub order: LimitOrderV4StructOutput,
    /// An interaction call data. ABI encoded set of makerAssetSuffix, takerAssetSuffix, makingAmountGetter, takingAmountGetter, predicate, permit, preInteraction, postInteraction.If extension exists then lowest 160 bits of the order salt must be equal to the lowest 160 bits of the extension hash
    pub extension: String,
    pub points: Option<Vec<AuctionPointOutput>>,
    /// Approximate amount of the takerAsset being requested by the maker in dst chain.
    pub approximate_taking_amount: String,
    /// shows if user received more than expected
    pub positive_surplus: String,
    /// Fills
    pub fills: Vec<FillOutputDto>,
    /// Unix timestamp in milliseconds
    pub auction_start_date: u64,
    /// Unix timestamp in milliseconds
    pub auction_duration: u64,
    /// Initial rate bump
    pub initial_rate_bump: u64,
    /// Unix timestamp in milliseconds
    pub created_at: u64,
    pub src_token_price_usd: serde_json::Value,
    pub dst_token_price_usd: serde_json::Value,
    pub cancel_tx: serde_json::Value,
    /// Identifier of the chain where the maker asset is located.
    pub src_chain_id: u64,
    /// Identifier of the chain where the taker asset is located.
    pub dst_chain_id: u64,
    /// Is order cancelable
    pub cancelable: bool,
    /// Identifier of the asset being requested by the maker in exchange in dst chain.
    pub taker_asset: String,
    /// TimeLocks without deployedAt
    pub time_locks: String,
}

/// Orders by hashes input
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrdersByHashesInput {
    pub order_hashes: Vec<String>,
}

impl ActiveOrdersParams {
    /// Create a new active orders params
    pub fn new() -> Self {
        Self {
            page: None,
            limit: None,
            src_chain: None,
            dst_chain: None,
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

    /// Set the source chain
    pub fn with_src_chain(mut self, src_chain: u64) -> Self {
        self.src_chain = Some(src_chain);
        self
    }

    /// Set the destination chain
    pub fn with_dst_chain(mut self, dst_chain: u64) -> Self {
        self.dst_chain = Some(dst_chain);
        self
    }
}

impl OrdersByMakerParams {
    /// Create a new orders by maker params
    pub fn new() -> Self {
        Self {
            page: None,
            limit: None,
            timestamp_from: None,
            timestamp_to: None,
            src_token: None,
            dst_token: None,
            with_token: None,
            dst_chain_id: None,
            src_chain_id: None,
            chain_id: None,
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

    /// Set the timestamp from
    pub fn with_timestamp_from(mut self, timestamp_from: u64) -> Self {
        self.timestamp_from = Some(timestamp_from);
        self
    }

    /// Set the timestamp to
    pub fn with_timestamp_to(mut self, timestamp_to: u64) -> Self {
        self.timestamp_to = Some(timestamp_to);
        self
    }

    /// Set the source token
    pub fn with_src_token(mut self, src_token: String) -> Self {
        self.src_token = Some(src_token);
        self
    }

    /// Set the destination token
    pub fn with_dst_token(mut self, dst_token: String) -> Self {
        self.dst_token = Some(dst_token);
        self
    }

    /// Set the with token
    pub fn with_token(mut self, with_token: String) -> Self {
        self.with_token = Some(with_token);
        self
    }

    /// Set the destination chain ID
    pub fn with_dst_chain_id(mut self, dst_chain_id: u64) -> Self {
        self.dst_chain_id = Some(dst_chain_id);
        self
    }

    /// Set the source chain ID
    pub fn with_src_chain_id(mut self, src_chain_id: u64) -> Self {
        self.src_chain_id = Some(src_chain_id);
        self
    }

    /// Set the chain ID
    pub fn with_chain_id(mut self, chain_id: u64) -> Self {
        self.chain_id = Some(chain_id);
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
            .with_limit(100)
            .with_src_chain(1)
            .with_dst_chain(137);

        assert_eq!(params.page, Some(1));
        assert_eq!(params.limit, Some(100));
        assert_eq!(params.src_chain, Some(1));
        assert_eq!(params.dst_chain, Some(137));
    }

    #[test]
    fn test_orders_by_maker_params_creation() {
        let params = OrdersByMakerParams::new()
            .with_page(1)
            .with_limit(50)
            .with_timestamp_from(1634025600000)
            .with_timestamp_to(1634112000000)
            .with_src_token("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string())
            .with_dst_token("0x2791bca1f2de4661ed88a30c99a7a9449aa84174".to_string())
            .with_chain_id(1);

        assert_eq!(params.page, Some(1));
        assert_eq!(params.limit, Some(50));
        assert_eq!(params.timestamp_from, Some(1634025600000));
        assert_eq!(params.timestamp_to, Some(1634112000000));
        assert_eq!(params.src_token, Some("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string()));
        assert_eq!(params.dst_token, Some("0x2791bca1f2de4661ed88a30c99a7a9449aa84174".to_string()));
        assert_eq!(params.chain_id, Some(1));
    }

    #[test]
    fn test_cross_chain_order_dto_creation() {
        let order = CrossChainOrderDto {
            salt: "42".to_string(),
            maker: "0x995BE1CA945174D5bA75410C1E658a41eB13a2FA".to_string(),
            receiver: "0x995BE1CA945174D5bA75410C1E658a41eB13a2FB".to_string(),
            maker_asset: "0x995BE1CA945174D5bA75410C1E658a41eB13a2FC".to_string(),
            taker_asset: "0x995BE1CA945174D5bA75410C1E658a41eB13a2FD".to_string(),
            making_amount: "100000000000000000".to_string(),
            taking_amount: "100000000000000000".to_string(),
            maker_traits: "0x".to_string(),
        };

        assert_eq!(order.salt, "42");
        assert_eq!(order.maker, "0x995BE1CA945174D5bA75410C1E658a41eB13a2FA");
        assert_eq!(order.receiver, "0x995BE1CA945174D5bA75410C1E658a41eB13a2FB");
        assert_eq!(order.maker_asset, "0x995BE1CA945174D5bA75410C1E658a41eB13a2FC");
        assert_eq!(order.taker_asset, "0x995BE1CA945174D5bA75410C1E658a41eB13a2FD");
        assert_eq!(order.making_amount, "100000000000000000");
        assert_eq!(order.taking_amount, "100000000000000000");
        assert_eq!(order.maker_traits, "0x");
    }

    #[test]
    fn test_immutables_creation() {
        let immutables = Immutables {
            order_hash: "0x496755a88564d8ded6759dff0252d3e6c3ef1fe42b4fa1bbc3f03bd2674f1078".to_string(),
            hashlock: "0x03f9ebf9075dfaae76c43b7443d07399609ffe24a5d435045fe4d3bf82d9fce4".to_string(),
            maker: "0xe75eD6F453c602Bd696cE27AF11565eDc9b46B0D".to_string(),
            taker: "0x00000000009E50a7dDb7a7B0e2ee6604fd120E49".to_string(),
            token: "0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string(),
            amount: "1000000000000000000".to_string(),
            safety_deposit: "50000000000000000000".to_string(),
            timelocks: "0x3000000020000000100000004000000030000000200000001".to_string(),
        };

        assert_eq!(immutables.order_hash, "0x496755a88564d8ded6759dff0252d3e6c3ef1fe42b4fa1bbc3f03bd2674f1078");
        assert_eq!(immutables.hashlock, "0x03f9ebf9075dfaae76c43b7443d07399609ffe24a5d435045fe4d3bf82d9fce4");
        assert_eq!(immutables.maker, "0xe75eD6F453c602Bd696cE27AF11565eDc9b46B0D");
        assert_eq!(immutables.taker, "0x00000000009E50a7dDb7a7B0e2ee6604fd120E49");
        assert_eq!(immutables.token, "0xdAC17F958D2ee523a2206206994597C13D831ec7");
        assert_eq!(immutables.amount, "1000000000000000000");
        assert_eq!(immutables.safety_deposit, "50000000000000000000");
        assert_eq!(immutables.timelocks, "0x3000000020000000100000004000000030000000200000001");
    }

    #[test]
    fn test_public_secret_creation() {
        let src_immutables = Immutables {
            order_hash: "0x496755a88564d8ded6759dff0252d3e6c3ef1fe42b4fa1bbc3f03bd2674f1078".to_string(),
            hashlock: "0x03f9ebf9075dfaae76c43b7443d07399609ffe24a5d435045fe4d3bf82d9fce4".to_string(),
            maker: "0xe75eD6F453c602Bd696cE27AF11565eDc9b46B0D".to_string(),
            taker: "0x00000000009E50a7dDb7a7B0e2ee6604fd120E49".to_string(),
            token: "0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string(),
            amount: "1000000000000000000".to_string(),
            safety_deposit: "50000000000000000000".to_string(),
            timelocks: "0x3000000020000000100000004000000030000000200000001".to_string(),
        };

        let dst_immutables = Immutables {
            order_hash: "0x496755a88564d8ded6759dff0252d3e6c3ef1fe42b4fa1bbc3f03bd2674f1079".to_string(),
            hashlock: "0x03f9ebf9075dfaae76c43b7443d07399609ffe24a5d435045fe4d3bf82d9fce5".to_string(),
            maker: "0xe75eD6F453c602Bd696cE27AF11565eDc9b46B0E".to_string(),
            taker: "0x00000000009E50a7dDb7a7B0e2ee6604fd120E4A".to_string(),
            token: "0xdAC17F958D2ee523a2206206994597C13D831ec8".to_string(),
            amount: "2000000000000000000".to_string(),
            safety_deposit: "60000000000000000000".to_string(),
            timelocks: "0x3000000020000000100000004000000030000000200000002".to_string(),
        };

        let public_secret = PublicSecret {
            idx: 1,
            secret: "0xdb475911f2d1c5df6b1fb959777ddd01c89d881175a3b9693ec884f18dcb5734".to_string(),
            src_immutables,
            dst_immutables,
        };

        assert_eq!(public_secret.idx, 1);
        assert_eq!(public_secret.secret, "0xdb475911f2d1c5df6b1fb959777ddd01c89d881175a3b9693ec884f18dcb5734");
        assert_eq!(public_secret.src_immutables.order_hash, "0x496755a88564d8ded6759dff0252d3e6c3ef1fe42b4fa1bbc3f03bd2674f1078");
        assert_eq!(public_secret.dst_immutables.order_hash, "0x496755a88564d8ded6759dff0252d3e6c3ef1fe42b4fa1bbc3f03bd2674f1079");
    }

    #[test]
    fn test_ready_to_accept_secret_fill_creation() {
        let fill = ReadyToAcceptSecretFill {
            idx: 1,
            src_escrow_deploy_tx_hash: "0x806039f5149065924ad52de616b50abff488c986716d052e9c160887bc09e559".to_string(),
            dst_escrow_deploy_tx_hash: "0x906039f5149065924ad52de616b50abff488c986716d052e9c160887bc09e560".to_string(),
        };

        assert_eq!(fill.idx, 1);
        assert_eq!(fill.src_escrow_deploy_tx_hash, "0x806039f5149065924ad52de616b50abff488c986716d052e9c160887bc09e559");
        assert_eq!(fill.dst_escrow_deploy_tx_hash, "0x906039f5149065924ad52de616b50abff488c986716d052e9c160887bc09e560");
    }

    #[test]
    fn test_escrow_factory_creation() {
        let escrow = EscrowFactory {
            address: "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".to_string(),
        };

        assert_eq!(escrow.address, "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee");
    }

    #[test]
    fn test_orders_by_hashes_input_creation() {
        let input = OrdersByHashesInput {
            order_hashes: vec![
                "0x10ea5bd12b2d04566e175de24c2df41a058bf16df4af3eb2fb9bff38a9da98e9".to_string(),
                "0x20ea5bd12b2d04566e175de24c2df41a058bf16df4af3eb2fb9bff38a9da98e8".to_string(),
                "0x30ea5bd12b2d04566e175de24c2df41a058bf16df4af3eb2fb9bff38a9da98e7".to_string(),
            ],
        };

        assert_eq!(input.order_hashes.len(), 3);
        assert_eq!(input.order_hashes[0], "0x10ea5bd12b2d04566e175de24c2df41a058bf16df4af3eb2fb9bff38a9da98e9");
        assert_eq!(input.order_hashes[1], "0x20ea5bd12b2d04566e175de24c2df41a058bf16df4af3eb2fb9bff38a9da98e8");
        assert_eq!(input.order_hashes[2], "0x30ea5bd12b2d04566e175de24c2df41a058bf16df4af3eb2fb9bff38a9da98e7");
    }
}
