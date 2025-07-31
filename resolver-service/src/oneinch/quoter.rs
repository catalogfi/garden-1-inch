use serde::{Deserialize, Serialize};
use anyhow::Result;
use reqwest::Client;

/// Quoter API client for 1inch Fusion+
pub struct QuoterClient {
    client: Client,
    base_url: String,
}

impl QuoterClient {
    /// Create a new quoter client
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Get quote details based on input data
    pub async fn get_quote(&self, params: QuoteParams) -> Result<GetQuoteOutput> {
        let url = format!("{}/quoter/v1.0/quote/receive", self.base_url);
        
        let response = self.client
            .get(&url)
            .query(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let quote: GetQuoteOutput = response.json().await?;
            Ok(quote)
        } else {
            Err(anyhow::anyhow!("Failed to get quote: {}", response.status()))
        }
    }

    /// Get quote with custom preset details
    pub async fn get_quote_with_custom_presets(
        &self,
        params: QuoteParams,
        custom_preset: CustomPresetParams,
    ) -> Result<GetQuoteOutput> {
        let url = format!("{}/quoter/v1.0/quote/receive", self.base_url);
        
        let response = self.client
            .post(&url)
            .query(&params)
            .json(&custom_preset)
            .send()
            .await?;

        if response.status().is_success() {
            let quote: GetQuoteOutput = response.json().await?;
            Ok(quote)
        } else {
            Err(anyhow::anyhow!("Failed to get quote with custom presets: {}", response.status()))
        }
    }

    /// Build order by given quote
    pub async fn build_quote_typed_data(&self, params: BuildOrderParams, body: BuildOrderBody) -> Result<BuildOrderOutput> {
        let url = format!("{}/quoter/v1.0/quote/build", self.base_url);
        
        let response = self.client
            .post(&url)
            .query(&params)
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            let output: BuildOrderOutput = response.json().await?;
            Ok(output)
        } else {
            Err(anyhow::anyhow!("Failed to build quote typed data: {}", response.status()))
        }
    }
}

/// Parameters for getting a quote
#[derive(Debug, Serialize)]
pub struct QuoteParams {
    /// Id of source chain
    pub src_chain: u64,
    /// Id of destination chain
    pub dst_chain: u64,
    /// Address of "SOURCE" token in source chain
    pub src_token_address: String,
    /// Address of "DESTINATION" token in destination chain
    pub dst_token_address: String,
    /// Amount to take from "SOURCE" token to get "DESTINATION" token
    pub amount: String,
    /// An address of the wallet or contract in source chain who will create Fusion order
    pub wallet_address: String,
    /// if enabled then get estimation from 1inch swap builder and generates quoteId, by default is false
    pub enable_estimate: bool,
    /// fee in bps format, 1% is equal to 100bps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<u64>,
    /// permit2 allowance transfer encoded call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_permit2: Option<String>,
    /// permit, user approval sign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permit: Option<String>,
}

/// Parameters for building an order
#[derive(Debug, Serialize)]
pub struct BuildOrderParams {
    /// Id of source chain
    pub src_chain: u64,
    /// Id of destination chain
    pub dst_chain: u64,
    /// Address of "SOURCE" token
    pub src_token_address: String,
    /// Address of "DESTINATION" token
    pub dst_token_address: String,
    /// Amount to take from "SOURCE" token to get "DESTINATION" token
    pub amount: String,
    /// An address of the wallet or contract who will create Fusion order
    pub wallet_address: String,
    /// fee in bps format, 1% is equal to 100bps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<u64>,
    /// Frontend or some other source selector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// permit2 allowance transfer encoded call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_permit2: Option<String>,
    /// Enabled flag allows to save quote for Mobile History
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_mobile: Option<String>,
    /// In case fee non zero -> the fee will be transferred to this address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_receiver: Option<String>,
    /// permit, user approval sign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permit: Option<String>,
    /// fast/medium/slow/custom
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
}

/// Custom preset parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomPresetParams {
    // This is an empty object in the OpenAPI spec
}

/// Build order body
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildOrderBody {
    /// quote from /receive
    pub quote: GetQuoteOutput,
    /// keccak256(secret)[]
    pub secrets_hash_list: Vec<String>,
}

/// Build order output
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildOrderOutput {
    /// EIP712 Typed Data
    pub typed_data: serde_json::Value,
    /// Hash of CrossChain order
    pub order_hash: String,
    /// CrossChain order extension
    pub extension: String,
}

/// Auction point
#[derive(Debug, Serialize, Deserialize)]
pub struct AuctionPoint {
    pub delay: u64,
    pub coefficient: f64,
}

/// Gas cost configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct GasCostConfig {
    pub gas_bump_estimate: u64,
    pub gas_price_estimate: String,
}

/// Preset configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Preset {
    pub auction_duration: u64,
    pub start_auction_in: u64,
    pub initial_rate_bump: u64,
    pub auction_start_amount: String,
    pub start_amount: String,
    pub auction_end_amount: String,
    pub exclusive_resolver: Option<serde_json::Value>,
    pub cost_in_dst_token: String,
    pub points: Vec<AuctionPoint>,
    pub allow_partial_fills: bool,
    pub allow_multiple_fills: bool,
    pub gas_cost: GasCostConfig,
    pub secrets_count: u64,
}

/// Quote presets
#[derive(Debug, Serialize, Deserialize)]
pub struct QuotePresets {
    pub fast: Preset,
    pub medium: Preset,
    pub slow: Preset,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Preset>,
}

/// Time locks configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeLocks {
    pub src_withdrawal: u64,
    pub src_public_withdrawal: u64,
    pub src_cancellation: u64,
    pub src_public_cancellation: u64,
    pub dst_withdrawal: u64,
    pub dst_public_withdrawal: u64,
    pub dst_cancellation: u64,
}

/// Token pair
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub src_token: String,
    pub dst_token: String,
}

/// Pair currency
#[derive(Debug, Serialize, Deserialize)]
pub struct PairCurrency {
    pub usd: TokenPair,
}

/// Get quote output
#[derive(Debug, Serialize, Deserialize)]
pub struct GetQuoteOutput {
    /// Current generated quote id, should be passed with order
    pub quote_id: serde_json::Value,
    pub src_token_amount: String,
    pub dst_token_amount: String,
    /// Various preset types which user can choose when using Fusion
    pub presets: QuotePresets,
    /// Escrow factory contract address at source chain
    pub src_escrow_factory: String,
    /// Escrow factory contract address at destination chain
    pub dst_escrow_factory: String,
    /// current executors whitelist addresses
    pub whitelist: Vec<String>,
    /// Timing config
    pub time_locks: TimeLocks,
    pub src_safety_deposit: String,
    pub dst_safety_deposit: String,
    /// suggested preset
    pub recommended_preset: String,
    pub prices: PairCurrency,
    pub volume: PairCurrency,
}

impl QuoteParams {
    /// Create a new quote params
    pub fn new(
        src_chain: u64,
        dst_chain: u64,
        src_token_address: String,
        dst_token_address: String,
        amount: String,
        wallet_address: String,
        enable_estimate: bool,
    ) -> Self {
        Self {
            src_chain,
            dst_chain,
            src_token_address,
            dst_token_address,
            amount,
            wallet_address,
            enable_estimate,
            fee: None,
            is_permit2: None,
            permit: None,
        }
    }

    /// Set the fee
    pub fn with_fee(mut self, fee: u64) -> Self {
        self.fee = Some(fee);
        self
    }

    /// Set permit2
    pub fn with_permit2(mut self, is_permit2: String) -> Self {
        self.is_permit2 = Some(is_permit2);
        self
    }

    /// Set permit
    pub fn with_permit(mut self, permit: String) -> Self {
        self.permit = Some(permit);
        self
    }
}

impl BuildOrderParams {
    /// Create a new build order params
    pub fn new(
        src_chain: u64,
        dst_chain: u64,
        src_token_address: String,
        dst_token_address: String,
        amount: String,
        wallet_address: String,
    ) -> Self {
        Self {
            src_chain,
            dst_chain,
            src_token_address,
            dst_token_address,
            amount,
            wallet_address,
            fee: None,
            source: None,
            is_permit2: None,
            is_mobile: None,
            fee_receiver: None,
            permit: None,
            preset: None,
        }
    }

    /// Set the fee
    pub fn with_fee(mut self, fee: u64) -> Self {
        self.fee = Some(fee);
        self
    }

    /// Set the source
    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    /// Set permit2
    pub fn with_permit2(mut self, is_permit2: String) -> Self {
        self.is_permit2 = Some(is_permit2);
        self
    }

    /// Set mobile flag
    pub fn with_mobile(mut self, is_mobile: String) -> Self {
        self.is_mobile = Some(is_mobile);
        self
    }

    /// Set fee receiver
    pub fn with_fee_receiver(mut self, fee_receiver: String) -> Self {
        self.fee_receiver = Some(fee_receiver);
        self
    }

    /// Set permit
    pub fn with_permit(mut self, permit: String) -> Self {
        self.permit = Some(permit);
        self
    }

    /// Set preset
    pub fn with_preset(mut self, preset: String) -> Self {
        self.preset = Some(preset);
        self
    }
}

impl BuildOrderBody {
    /// Create a new build order body
    pub fn new(quote: GetQuoteOutput, secrets_hash_list: Vec<String>) -> Self {
        Self {
            quote,
            secrets_hash_list,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_params_creation() {
        let params = QuoteParams::new(
            1, // Ethereum mainnet
            137, // Polygon
            "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string(), // WETH
            "0x2791bca1f2de4661ed88a30c99a7a9449aa84174".to_string(), // USDC
            "1000000000000000000".to_string(), // 1 WETH
            "0x0000000000000000000000000000000000000000".to_string(),
            false,
        )
        .with_fee(100) // 1% fee
        .with_permit("0xpermit".to_string());

        assert_eq!(params.src_chain, 1);
        assert_eq!(params.dst_chain, 137);
        assert_eq!(params.src_token_address, "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
        assert_eq!(params.dst_token_address, "0x2791bca1f2de4661ed88a30c99a7a9449aa84174");
        assert_eq!(params.amount, "1000000000000000000");
        assert_eq!(params.wallet_address, "0x0000000000000000000000000000000000000000");
        assert_eq!(params.enable_estimate, false);
        assert_eq!(params.fee, Some(100));
        assert_eq!(params.permit, Some("0xpermit".to_string()));
    }

    #[test]
    fn test_build_order_params_creation() {
        let params = BuildOrderParams::new(
            1, // Ethereum mainnet
            137, // Polygon
            "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string(), // WETH
            "0x2791bca1f2de4661ed88a30c99a7a9449aa84174".to_string(), // USDC
            "1000000000000000000".to_string(), // 1 WETH
            "0x0000000000000000000000000000000000000000".to_string(),
        )
        .with_fee(50) // 0.5% fee
        .with_source("Frontend".to_string())
        .with_preset("fast".to_string());

        assert_eq!(params.src_chain, 1);
        assert_eq!(params.dst_chain, 137);
        assert_eq!(params.src_token_address, "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
        assert_eq!(params.dst_token_address, "0x2791bca1f2de4661ed88a30c99a7a9449aa84174");
        assert_eq!(params.amount, "1000000000000000000");
        assert_eq!(params.wallet_address, "0x0000000000000000000000000000000000000000");
        assert_eq!(params.fee, Some(50));
        assert_eq!(params.source, Some("Frontend".to_string()));
        assert_eq!(params.preset, Some("fast".to_string()));
    }

    #[test]
    fn test_build_order_body_creation() {
        // Create a mock quote output
        let quote = GetQuoteOutput {
            quote_id: serde_json::json!({"id": "test_quote"}),
            src_token_amount: "1000000000000000000".to_string(),
            dst_token_amount: "2500000000".to_string(),
            presets: QuotePresets {
                fast: Preset {
                    auction_duration: 100,
                    start_auction_in: 2,
                    initial_rate_bump: 1000,
                    auction_start_amount: "2500000000".to_string(),
                    start_amount: "2500000000".to_string(),
                    auction_end_amount: "2500000000".to_string(),
                    exclusive_resolver: None,
                    cost_in_dst_token: "5000000".to_string(),
                    points: vec![AuctionPoint { delay: 12, coefficient: 455.0 }],
                    allow_partial_fills: false,
                    allow_multiple_fills: false,
                    gas_cost: GasCostConfig {
                        gas_bump_estimate: 54,
                        gas_price_estimate: "1231".to_string(),
                    },
                    secrets_count: 1,
                },
                medium: Preset {
                    auction_duration: 200,
                    start_auction_in: 5,
                    initial_rate_bump: 800,
                    auction_start_amount: "2500000000".to_string(),
                    start_amount: "2500000000".to_string(),
                    auction_end_amount: "2500000000".to_string(),
                    exclusive_resolver: None,
                    cost_in_dst_token: "4000000".to_string(),
                    points: vec![AuctionPoint { delay: 15, coefficient: 400.0 }],
                    allow_partial_fills: true,
                    allow_multiple_fills: false,
                    gas_cost: GasCostConfig {
                        gas_bump_estimate: 45,
                        gas_price_estimate: "1100".to_string(),
                    },
                    secrets_count: 1,
                },
                slow: Preset {
                    auction_duration: 300,
                    start_auction_in: 10,
                    initial_rate_bump: 600,
                    auction_start_amount: "2500000000".to_string(),
                    start_amount: "2500000000".to_string(),
                    auction_end_amount: "2500000000".to_string(),
                    exclusive_resolver: None,
                    cost_in_dst_token: "3000000".to_string(),
                    points: vec![AuctionPoint { delay: 20, coefficient: 350.0 }],
                    allow_partial_fills: true,
                    allow_multiple_fills: true,
                    gas_cost: GasCostConfig {
                        gas_bump_estimate: 35,
                        gas_price_estimate: "1000".to_string(),
                    },
                    secrets_count: 2,
                },
                custom: None,
            },
            src_escrow_factory: "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".to_string(),
            dst_escrow_factory: "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".to_string(),
            whitelist: vec!["0x1d3b67bca8935cb510c8d18bd45f0b94f54a9681".to_string()],
            time_locks: TimeLocks {
                src_withdrawal: 20,
                src_public_withdrawal: 21,
                src_cancellation: 22,
                src_public_cancellation: 23,
                dst_withdrawal: 24,
                dst_public_withdrawal: 25,
                dst_cancellation: 26,
            },
            src_safety_deposit: "123".to_string(),
            dst_safety_deposit: "123".to_string(),
            recommended_preset: "fast".to_string(),
            prices: PairCurrency {
                usd: TokenPair {
                    src_token: "2505.44210175".to_string(),
                    dst_token: "1.0008429148729692".to_string(),
                },
            },
            volume: PairCurrency {
                usd: TokenPair {
                    src_token: "250.544210175".to_string(),
                    dst_token: "250.62930624631504754367".to_string(),
                },
            },
        };

        let secrets_hash_list = vec![
            "0x315b47a8c3780434b153667588db4ca628526e20000000000000000000000000".to_string(),
        ];

        let body = BuildOrderBody::new(quote, secrets_hash_list);

        assert_eq!(body.secrets_hash_list.len(), 1);
        assert_eq!(body.quote.recommended_preset, "fast");
        assert_eq!(body.quote.src_token_amount, "1000000000000000000");
        assert_eq!(body.quote.dst_token_amount, "2500000000");
    }
}
