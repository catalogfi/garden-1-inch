use serde::{Deserialize, Serialize};
use anyhow::Result;
use reqwest::Client;

/// Relayer API client for 1inch Fusion+
pub struct RelayerClient {
    client: Client,
    base_url: String,
}

impl RelayerClient {
    /// Create a new relayer client
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Submit a single cross chain order
    pub async fn submit_order(&self, order: SignedOrderInput) -> Result<()> {
        let url = format!("{}/relayer/v1.0/submit", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&order)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to submit order: {}", response.status()))
        }
    }

    /// Submit multiple cross chain orders
    pub async fn submit_many_orders(&self, order_hashes: Vec<String>) -> Result<()> {
        let url = format!("{}/relayer/v1.0/submit/many", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&order_hashes)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to submit orders: {}", response.status()))
        }
    }

    /// Submit a secret for order fill
    pub async fn submit_secret(&self, secret: SecretInput) -> Result<()> {
        let url = format!("{}/relayer/v1.0/submit/secret", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&secret)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to submit secret: {}", response.status()))
        }
    }
}

/// Order input data structure
#[derive(Debug, Serialize, Deserialize)]
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
    #[serde(default = "default_receiver")]
    pub receiver: String,
    /// Order maker's token amount
    pub making_amount: String,
    /// Order taker's token amount
    pub taking_amount: String,
    /// Includes flags like: allow multiple fills, is partial fill allowed or not, price improvement, nonce, deadline etc.
    #[serde(default = "default_maker_traits")]
    pub maker_traits: String,
}

fn default_receiver() -> String {
    "0x0000000000000000000000000000000000000001".to_string()
}

fn default_maker_traits() -> String {
    "0".to_string()
}

/// Signed order input for submission
#[derive(Debug, Serialize, Deserialize)]
pub struct SignedOrderInput {
    /// Cross chain order data
    pub order: OrderInput,
    /// Source chain id
    pub src_chain_id: u64,
    /// Signature of the cross chain order typed data (using signTypedData v4)
    pub signature: String,
    /// An interaction call data. ABI encoded a set of makerAssetSuffix, takerAssetSuffix, makingAmountGetter, takingAmountGetter, predicate, permit, preInteraction, postInteraction
    pub extension: String,
    /// Quote id of the quote with presets
    pub quote_id: String,
    /// Secret Hashes, required for order with multiple fills allowed. keccak256(secret(idx))
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_hashes: Option<Vec<String>>,
}

/// Secret input for order fill
#[derive(Debug, Serialize, Deserialize)]
pub struct SecretInput {
    /// A secret for the fill hashlock
    pub secret: String,
    /// Order hash
    pub order_hash: String,
}

impl OrderInput {
    /// Create a new order input
    pub fn new(
        salt: String,
        maker_asset: String,
        taker_asset: String,
        maker: String,
        making_amount: String,
        taking_amount: String,
    ) -> Self {
        Self {
            salt,
            maker_asset,
            taker_asset,
            maker,
            receiver: default_receiver(),
            making_amount,
            taking_amount,
            maker_traits: default_maker_traits(),
        }
    }

    /// Set the receiver address
    pub fn with_receiver(mut self, receiver: String) -> Self {
        self.receiver = receiver;
        self
    }

    /// Set the maker traits
    pub fn with_maker_traits(mut self, maker_traits: String) -> Self {
        self.maker_traits = maker_traits;
        self
    }
}

impl SignedOrderInput {
    /// Create a new signed order input
    pub fn new(
        order: OrderInput,
        src_chain_id: u64,
        signature: String,
        extension: String,
        quote_id: String,
    ) -> Self {
        Self {
            order,
            src_chain_id,
            signature,
            extension,
            quote_id,
            secret_hashes: None,
        }
    }

    /// Set secret hashes for multiple fills
    pub fn with_secret_hashes(mut self, secret_hashes: Vec<String>) -> Self {
        self.secret_hashes = Some(secret_hashes);
        self
    }
}

impl SecretInput {
    /// Create a new secret input
    pub fn new(secret: String, order_hash: String) -> Self {
        Self {
            secret,
            order_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_input_creation() {
        let order = OrderInput::new(
            "42".to_string(),
            "0x0000000000000000000000000000000000000001".to_string(),
            "0x0000000000000000000000000000000000000002".to_string(),
            "0x0000000000000000000000000000000000000003".to_string(),
            "100000000000000000000".to_string(),
            "100000000000000000000".to_string(),
        );

        assert_eq!(order.salt, "42");
        assert_eq!(order.maker_asset, "0x0000000000000000000000000000000000000001");
        assert_eq!(order.taker_asset, "0x0000000000000000000000000000000000000002");
        assert_eq!(order.maker, "0x0000000000000000000000000000000000000003");
        assert_eq!(order.making_amount, "100000000000000000000");
        assert_eq!(order.taking_amount, "100000000000000000000");
        assert_eq!(order.receiver, "0x0000000000000000000000000000000000000001");
        assert_eq!(order.maker_traits, "0");
    }

    #[test]
    fn test_signed_order_input_creation() {
        let order = OrderInput::new(
            "42".to_string(),
            "0x0000000000000000000000000000000000000001".to_string(),
            "0x0000000000000000000000000000000000000002".to_string(),
            "0x0000000000000000000000000000000000000003".to_string(),
            "100000000000000000000".to_string(),
            "100000000000000000000".to_string(),
        );

        let signed_order = SignedOrderInput::new(
            order,
            1,
            "0xsignature".to_string(),
            "0x".to_string(),
            "quote_123".to_string(),
        );

        assert_eq!(signed_order.src_chain_id, 1);
        assert_eq!(signed_order.signature, "0xsignature");
        assert_eq!(signed_order.extension, "0x");
        assert_eq!(signed_order.quote_id, "quote_123");
        assert!(signed_order.secret_hashes.is_none());
    }

    #[test]
    fn test_secret_input_creation() {
        let secret = SecretInput::new(
            "secret_value".to_string(),
            "order_hash_123".to_string(),
        );

        assert_eq!(secret.secret, "secret_value");
        assert_eq!(secret.order_hash, "order_hash_123");
    }
}
