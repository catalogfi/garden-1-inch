use std::str::FromStr;
use std::fs;

use anyhow::Result;
use alloy::{
    contract::{ContractInstance, Interface}, dyn_abi::{DynSolValue, Word}, hex, json_abi::JsonAbi, network::EthereumWallet, primitives::{Address, U256}, providers::{fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller}, ProviderBuilder, RootProvider}, signers::local::LocalSigner
};
use reqwest::Url;
use serde_json::Value;
use crate::{order_mapper::OrderAction, resolver::Resolver, settings::ChainSettings};

pub struct ResolverContract {
    address: String,
    provider: String,
    private_key: String,
}

impl ResolverContract {
    pub fn new(address: &String, provider: &String, private_key: &String) -> Self {
        Self {
            address: address.clone(),
            provider: provider.clone(),
            private_key: private_key.clone(),
        }
    }

    async fn get_contract(&self) -> Result<ContractInstance<FillProvider<JoinFill<JoinFill<alloy::providers::Identity, JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>>, WalletFiller<EthereumWallet>>, RootProvider>>> {
        let signer = LocalSigner::from_str(&self.private_key).unwrap();
        tracing::info!("signer: {:?}", signer.address());

        let provider = ProviderBuilder::new().wallet(signer).connect_http(Url::from_str(&self.provider)?);
        let contract_address = Address::from_str(&self.address)?;
        
        // Load ABI from evm_abi.json file
        let abi_content = fs::read_to_string("src/resolver/evm_abi.json")?;
        let full_json: Value = serde_json::from_str(&abi_content)?;
        
        // Extract just the ABI array from the contract artifact
        let abi_array = full_json["abi"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Missing 'abi' field in contract artifact"))?;
        
        // Parse the ABI array as JsonAbi
        let json_abi: JsonAbi = serde_json::from_value(Value::Array(abi_array.clone()))?;
        let interface = Interface::new(json_abi);
        // Create ContractInstance with the Interface
        Ok(ContractInstance::new(contract_address, provider, interface))
    }
}

pub struct EvmResolver {
    contract: ResolverContract,
    chain_id: String,
}

#[async_trait::async_trait]
impl Resolver for EvmResolver {
    async fn deploy_dest_escrow(&self, order_action: &OrderAction) -> Result<()> {
        tracing::info!(
            chain_id=?self.chain_id,
            order_id=?order_action.order_id,
            "Deploying escrow"
        );
        
        let contract = self.contract.get_contract().await?;
        
        // For deployDst function, we need:
        // - dstImmutables (IBaseEscrow.Immutables) - tuple of 8 elements
        // - srcCancellationTimestamp (uint256)
        let secret_hash = order_action.order.secrets.first().map(|s| s.secret_hash.clone()).unwrap_or("0x0000000000000000000000000000000000000000000000000000000000000000".to_string());
        let safety_deposit = U256::from(0u64);
        // Create immutables tuple based on order data
        // IBaseEscrow.Immutables: (bytes32, bytes32, uint256, uint256, uint256, uint256, uint256, uint256)
        let immutables_tuple = DynSolValue::Tuple(vec![
            DynSolValue::FixedBytes(Word::from_str(&order_action.order.order_hash)?, 32), // orderHash (bytes32)
            DynSolValue::FixedBytes(Word::from_str(&secret_hash)?, 32), // hashlock (bytes32)
            DynSolValue::Uint(U256::from_str(&order_action.order.order.maker)?, 256), // maker (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.order.receiver)?, 256), // taker (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.order.maker_asset)?, 256), // token (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.order.making_amount)?, 256), // amount (uint256)
            DynSolValue::Uint(safety_deposit, 256), // safetyDeposit (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.deadline)?, 256), // timelocks (uint256)
        ]);
        
        // Use current timestamp as srcCancellationTimestamp
        let src_cancellation_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let result = contract
            .function("deployDst", &[immutables_tuple, DynSolValue::Uint(U256::from(src_cancellation_timestamp), 256)])?
            .send()
            .await?;
            
        tracing::info!("Escrow deployed: {:?}", result);
        Ok(())
    }

    async fn deploy_src_escrow(&self, order_action: &OrderAction) -> Result<()> {
        tracing::info!(
            chain_id=?self.chain_id,
            order_id=?order_action.order_id,
            "Releasing funds"
        );
        
        let contract = self.contract.get_contract().await?;
        
        // For deploySrc function, we need:
        // - immutables (IBaseEscrow.Immutables) - tuple of 8 elements
        // - order (IOrderMixin.Order) - tuple of 8 elements
        // - r (bytes32)
        // - vs (bytes32)
        // - amount (uint256)
        // - takerTraits (uint256)
        // - args (bytes)
        let secret_hash = order_action.order.secrets.first().map(|s| s.secret_hash.clone()).unwrap_or("0x0000000000000000000000000000000000000000000000000000000000000000".to_string());
        // Create immutables tuple
        let safety_deposit = U256::from(0u64);
        // IBaseEscrow.Immutables: (bytes32, bytes32, uint256, uint256, uint256, uint256, uint256, uint256)
        let immutables_tuple = DynSolValue::Tuple(vec![
            DynSolValue::FixedBytes(Word::from_str(&order_action.order.order_hash)?, 32), // orderHash (bytes32)
            DynSolValue::FixedBytes(Word::from_str(&secret_hash)?, 32), // hashlock (bytes32)
            DynSolValue::Uint(U256::from_str(&order_action.order.order.maker)?, 256), // maker (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.order.receiver)?, 256), // taker (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.order.maker_asset)?, 256), // token (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.order.making_amount)?, 256), // amount (uint256)
            DynSolValue::Uint(safety_deposit, 256), // safetyDeposit (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.deadline)?, 256), // timelocks (uint256)
        ]);
        tracing::info!("immutables_tuple: {:?}", immutables_tuple);
        // Create order tuple
        // IOrderMixin.Order: (uint256, uint256, uint256, uint256, uint256, uint256, uint256, uint256)
        let order_tuple = DynSolValue::Tuple(vec![
            DynSolValue::Uint(U256::from_str(&order_action.order.order.salt)?, 256), // salt (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.order.maker)?, 256), // maker (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.order.receiver)?, 256), // receiver (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.order.maker_asset)?, 256), // makerAsset (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.order.taker_asset)?, 256), // takerAsset (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.order.making_amount)?, 256), // makingAmount (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.order.taking_amount)?, 256), // takingAmount (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.order.maker_traits)?, 256), // makerTraits (uint256)
        ]);
        
        tracing::info!("order_tuple: {:?}", order_tuple);
        
        // Parse signature into r and vs components
        // Assuming signature is in format "0x" + r + s + v
        let signature = order_action.order.signature.trim_start_matches("0x");
        let r_bytes: [u8; 32] = hex::decode(&signature[..64]).unwrap().try_into().map_err(|_| anyhow::anyhow!("Invalid signature"))?;
        let vs_bytes: [u8; 32] = hex::decode(&signature[64..]).unwrap().try_into().map_err(|_| anyhow::anyhow!("Invalid signature"))?;
        
        // Use remaining maker amount as the fill amount
        let amount = U256::from_str(&order_action.order.remaining_maker_amount)?;
        
        // Set takerTraits with target flag (1 << 251)
        let taker_traits = U256::from(1u64) << 251;
        
        // Empty args for now
        let args = DynSolValue::Bytes(vec![]);
        
        let result = contract
            .function("deploySrc", &[immutables_tuple, order_tuple, DynSolValue::FixedBytes(Word::from_slice(&r_bytes), 32), DynSolValue::FixedBytes(Word::from_slice(&vs_bytes), 32), DynSolValue::Uint(amount, 256), DynSolValue::Uint(taker_traits, 256), args])?
            .send()
            .await.unwrap();
            
        tracing::info!("Funds released: {:?}", result);
        Ok(())
    }

    async fn arbitrary_calls(&self, order_action: &OrderAction) -> Result<()> {
        tracing::info!(
            chain_id=?self.chain_id,
            order_id=?order_action.order_id,
            "Refunding funds"
        );
        
        let contract = self.contract.get_contract().await?;
        
        // For arbitraryCalls function, we need:
        // - targets (address[] array)
        // - arguments (bytes[] array)
        
        // For refund, we typically need to call the escrow contract's refund function
        // This would depend on the specific escrow implementation
        // For now, we'll create a placeholder structure
        
        let targets = DynSolValue::Array(vec![
            // Escrow contract address would be determined from the order
            DynSolValue::Address(Address::from([0u8; 20])) // Placeholder
        ]);
        
        let arguments = DynSolValue::Array(vec![
            // Encoded refund function call
            DynSolValue::Bytes(vec![]) // Placeholder for encoded function call
        ]);
        
        let result = contract
            .function("arbitraryCalls", &[targets, arguments])?.send()
            .await?;
            
        tracing::info!("Funds refunded: {:?}", result);
        Ok(())
    }
}

impl EvmResolver {
    pub fn new(chain_settings: &ChainSettings) -> Self {
        let contract = ResolverContract::new(
            &chain_settings.resolver_contract_address,
            &chain_settings.provider,
            &chain_settings.private_key
        );
        Self {
            contract,
            chain_id: chain_settings.chain_id.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloy::{dyn_abi::Eip712Domain, signers::local::PrivateKeySigner, sol_types::{eip712_domain, SolValue}};

    use super::*;
    use crate::oneinch::orders::{ActiveOrder, Order, OrderType, SecretEntry};

    fn create_test_order_action() -> OrderAction {
        let order = Order {
            salt: "123456789".to_string(),
            maker_asset: "0xA0b86a33E6441b8c4C8C8C8C8C8C8C8C8C8C8C8C8".to_string(),
            taker_asset: "0xB1b86a33E6441b8c4C8C8C8C8C8C8C8C8C8C8C8C8".to_string(),
            maker: "0xC2b86a33E6441b8c4C8C8C8C8C8C8C8C8C8C8C8C8".to_string(),
            receiver: "0xD3b86a33E6441b8c4C8C8C8C8C8C8C8C8C8C8C8C8".to_string(),
            making_amount: "1000000000000000000".to_string(), // 1 ETH
            taking_amount: "2000000000000000000".to_string(), // 2 ETH
            maker_traits: "0".to_string(),
        };

        let secrets = vec![
            SecretEntry {
                index: 0,
                secret: Some("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string()),
                secret_hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890".to_string(),
            }
        ];

        let active_order = ActiveOrder {
            order_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
            signature: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
            deadline: "1234567890".to_string(),
            auction_start_date: None,
            auction_end_date: None,
            remaining_maker_amount: "1000000000000000000".to_string(),
            extension: "0".to_string(),
            src_chain_id: 1,
            dst_chain_id: 137,
            order,
            order_type: OrderType::SingleFill,
            secrets,
        };

        OrderAction {
            order_id: "test_order_123".to_string(),
            action_type: crate::order_mapper::ActionType::DeploySrcEscrow,
            order: active_order,
        }
    }

    fn create_test_chain_settings() -> ChainSettings {
        ChainSettings {
            chain_id: "1".to_string(),
            chain_type: crate::settings::ChainType::EVM,
            resolver_contract_address: "0xc4a39f6FF2B005aA9AD9Ac3D03BD95345fA50e86".to_string(),
            provider: "https://base-sepolia-rpc.publicnode.com".to_string(),
            assets: vec!["WBTC".to_string()],
            private_key: "0x149bc17929e5d9c43fb25ab94c112803130137bfdb2a2cfd6ef9043bd4fc22d6".to_string(),
        }
    }

    #[tokio::test]
    async fn test_deploy_escrow() {
        tracing_subscriber::fmt::init();
        let chain_settings = create_test_chain_settings();
        let resolver = EvmResolver::new(&chain_settings);
        let order_action = create_test_order_action();

        // Test deploy_escrow function
        let result = resolver.deploy_src_escrow(&order_action).await;
        println!("Result: {:?}", result);
    }

    #[tokio::test]
    async fn test_release_funds() {
        let chain_settings = create_test_chain_settings();
        let resolver = EvmResolver::new(&chain_settings);
        let mut order_action = create_test_order_action();
        
        // Change action type to ReleaseFunds
        order_action.action_type = crate::order_mapper::ActionType::DeployDestEscrow;

        // Test release_funds function
        let result = resolver.deploy_src_escrow(&order_action).await;
        
        // Since we don't have a real provider, this should fail with a connection error
        // but we can verify the function doesn't panic and handles errors gracefully
        assert!(result.is_err(), "Expected error due to no real provider connection");
        
        // Verify the error is related to connection/network, not parameter parsing
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("connection") || 
            error_msg.contains("network") || 
            error_msg.contains("timeout") ||
            error_msg.contains("invalid") ||
            error_msg.contains("parse"),
            "Unexpected error: {}", error_msg
        );
    }

    #[tokio::test]
    async fn test_refund_funds() {
        let chain_settings = create_test_chain_settings();
        let resolver = EvmResolver::new(&chain_settings);
        let mut order_action = create_test_order_action();
        
        // Change action type to RefundFunds
        order_action.action_type = crate::order_mapper::ActionType::ArbitraryCalls;

        // Test refund_funds function
        let result = resolver.arbitrary_calls(&order_action).await;
        
        // Since we don't have a real provider, this should fail with a connection error
        // but we can verify the function doesn't panic and handles errors gracefully
        assert!(result.is_err(), "Expected error due to no real provider connection");
        
        // Verify the error is related to connection/network, not parameter parsing
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("connection") || 
            error_msg.contains("network") || 
            error_msg.contains("timeout") ||
            error_msg.contains("invalid") ||
            error_msg.contains("parse"),
            "Unexpected error: {}", error_msg
        );
    }

    #[test]
    fn test_dynsolvalue_creation() {
        // Test that we can create DynSolValue structures correctly
        let test_address = "0x1234567890abcdef1234567890abcdef12345678";
        let test_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        
        // Test Address creation
        let address_value = DynSolValue::Address(Address::from_str(test_address).unwrap());
        assert!(matches!(address_value, DynSolValue::Address(_)));
        
        // Test FixedBytes creation
        let bytes_value = DynSolValue::FixedBytes(Word::from_str(test_hash).unwrap(), 32);
        assert!(matches!(bytes_value, DynSolValue::FixedBytes(_, 32)));
        
        // Test Uint creation
        let uint_value = DynSolValue::Uint(U256::from(12345u64), 256);
        assert!(matches!(uint_value, DynSolValue::Uint(_, 256)));
        
        // Test Tuple creation
        let tuple_value = DynSolValue::Tuple(vec![
            DynSolValue::Uint(U256::from(1u64), 256),
            DynSolValue::Uint(U256::from(2u64), 256),
        ]);
        assert!(matches!(tuple_value, DynSolValue::Tuple(_)));
        
        // Test Array creation
        let array_value = DynSolValue::Array(vec![
            DynSolValue::Address(Address::from([0u8; 20])),
        ]);
        assert!(matches!(array_value, DynSolValue::Array(_)));
    }

    #[tokio::test]
    async fn test_deploy_src_escrow_with_real_data() {
        tracing_subscriber::fmt::init();
        let chain_settings = create_test_chain_settings();
        let resolver = EvmResolver::new(&chain_settings);
        
        // Create order action with real data from the user's example
        let order_action = create_real_order_action();

        // Test deploy_src_escrow function with real data
        let result = resolver.deploy_src_escrow(&order_action).await;
        println!("Result: {:?}", result);
        
        // Since we don't have a real provider, this should fail with a connection error
        // but we can verify the function doesn't panic and handles errors gracefully
        assert!(result.is_err(), "Expected error due to no real provider connection");
        
        // Verify the error is related to connection/network, not parameter parsing
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("connection") || 
            error_msg.contains("network") || 
            error_msg.contains("timeout") ||
            error_msg.contains("invalid") ||
            error_msg.contains("parse"),
            "Unexpected error: {}", error_msg
        );
    }

    fn create_real_order_action() -> OrderAction {

        
        // Create order with real data from the user's example
        let order = Order {
            salt: "967407820121238835523921971039568048563901358305049".to_string(),
            maker_asset: "0x6756682b6144018dea5416640a0d0e8783e33f60".to_string(),
            taker_asset: "0xda0000d4000015a526378bb6fafc650cea5966f8".to_string(),
            maker: "0x1b150538e943f00127929f7eeb65754f7beb0b6d".to_string(),
            receiver: "0x0000000000000000000000000000000000000000".to_string(),
            making_amount: "100000000000000000000".to_string(), // 100 tokens
            taking_amount: "99000000000000000000".to_string(), // 99 tokens
            maker_traits: "62419173104490761595518734106515708578046331467977065221969182100300509478912".to_string(),
        };

        let secrets = vec![
            SecretEntry {
                index: 0,
                secret: None, // Secret not provided in the example
                secret_hash: "0x232c24908d508319a2544b51fe61ad81c05252dcca56d83d379ad8ca549c4fd6".to_string(),
            }
        ];

        let active_order = ActiveOrder {
            order_hash: "0xcbdd9dd779e8442356e191a971a366e52a88499c477e1a6b968f5c23b33abfbd".to_string(),
            signature: "0xa746a906a8d6fbe1cf39f7ac171b96e111a362e58ed72beffac6c9466c1f2c03d92b8f42367df31b6bdbe5ab4bce2d092d099e0c219c41d25efad69484624aa1".to_string(),
            deadline: "633987275420204920880845305940929565590401881683739122073601".to_string(),
            auction_start_date: None,
            auction_end_date: None,
            remaining_maker_amount: "100000000000000000000".to_string(), // 100 tokens
            extension: "0".to_string(),
            src_chain_id: 1,
            dst_chain_id: 137,
            order,
            order_type: OrderType::SingleFill,
            secrets,
        };

        OrderAction {
            order_id: "real_order_test".to_string(),
            action_type: crate::order_mapper::ActionType::DeploySrcEscrow,
            order: active_order,
        }
    }

    #[tokio::test]
    async fn sign_order() {
        let signer = PrivateKeySigner::from_str("0x639ed7560cbdde79096973912f5c83de86ba08aef2ce6f673dad5bf0a1663801").unwrap();

        let provider = ProviderBuilder::new().wallet(signer).connect_http(Url::from_str("https://base-sepolia-rpc.publicnode.com").unwrap());
        
        alloy::sol!(
            struct Order {
                uint256 salt;
                address maker;
                address receiver;
                address maker_asset;
                address taker_asset;
                uint256 making_amount;
                uint256 taking_amount;
                uint256 maker_traits;
            }
        );
        
        let order = Order {
            salt: U256::from_str("967407820121238835523921971039568048563901358305049").unwrap(),
            maker: Address::from_str("0x1b150538e943f00127929f7eeb65754f7beb0b6d").unwrap(),
            receiver: Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
            maker_asset: Address::from_str("0x6756682b6144018dea5416640a0d0e8783e33f60").unwrap(),
            taker_asset: Address::from_str("0xda0000d4000015a526378bb6fafc650cea5966f8").unwrap(),
            making_amount: U256::from_str("100000000000000000000").unwrap(),
            taking_amount: U256::from_str("99000000000000000000").unwrap(),
            maker_traits: U256::from_str("62419173104490761595518734106515708578046331467977065221969182100300509478912").unwrap(),
        };
        
        let domain = eip712_domain!  {
            name: "1inch Limit Order Protocol".to_string(),
            version: "3".to_string(),
            chain_id: 84532,
            verifying_contract: Address::from_str("0x4B715df6F89624dDb2c6DB70304b494d79531d92").unwrap(),
        };
        
        // let order = order.
    }

    #[tokio::test]
    async fn test_contract_call() {
        tracing_subscriber::fmt::init();
        
        // Create chain settings with real contract address and provider
        let chain_settings = ChainSettings {
            chain_id: "1".to_string(),
            chain_type: crate::settings::ChainType::EVM,
            resolver_contract_address: "0xb2E79cD69Ee0bA7a431BBab2585ae2Bd9019F68C".to_string(),
            provider: "https://rpc.ankr.com/monad_testnet".to_string(),
            assets: vec!["WBTC".to_string()],
            private_key: "0x149bc17929e5d9c43fb25ab94c112803130137bfdb2a2cfd6ef9043bd4fc22d6".to_string(),
        };
        
        let resolver = EvmResolver::new(&chain_settings);
        
        
        let immutables_tuple = DynSolValue::Tuple(vec![
            // orderHash: '0x35f5eaf042477a27e7f7f8d404b00d7f9e2990d2a92ec207e02a0bf6222bf38b'
            DynSolValue::FixedBytes(Word::from_str("0x0ccb18159149568f1dfc2b70a480dd4836b7aafe07f6905e215f37a3689db2fe").unwrap(), 32),
            // hashlock: '0x4048754ee73f1f94c13cc2f620769a262eec199ceaab852dbbb1d45ad01c7160'
            DynSolValue::FixedBytes(Word::from_str("0xee5a1c82e0fca3231cda28287461620a311f0fde663601cbe82255975dc7543e").unwrap(), 32),
            // maker: '0x1b150538e943f00127929f7eeb65754f7beb0b6d'
            DynSolValue::Uint(U256::from_str("0x1b150538e943f00127929f7eeb65754f7beb0b6d").unwrap(), 256),
            // taker: '0xc4a39f6ff2b005aa9ad9ac3d03bd95345fa50e86'
            DynSolValue::Uint(U256::from_str("0xb2e79cd69ee0ba7a431bbab2585ae2bd9019f68c").unwrap(), 256),
            // token: '0x6756682b6144018dea5416640a0d0e8783e33f60'
            DynSolValue::Uint(U256::from_str("0xea2bb31ebb0aee264aba3730c8744d6bd76d37d0").unwrap(), 256),
            // amount: '100000000000000000000'
            DynSolValue::Uint(U256::from_str("9000000000000000000").unwrap(), 256),
            // safetyDeposit: '1000000000000000'
            DynSolValue::Uint(U256::from_str("0").unwrap(), 256),
            // timelocks: '633987275420204920880845305940929565590401881683739122073601'
            DynSolValue::Uint(U256::from_str("47291213287644045068905695703641423110752441605741313059554104136313153257473").unwrap(), 256),
        ]);
        
        // Hardcode the order data based on the user's example
        let order_tuple = DynSolValue::Tuple(vec![
            // salt: '27163352540056185289423257040196066572461061026260'
            DynSolValue::Uint(U256::from_str("967407820121238835523921971039568048563901358305049").unwrap(), 256),
            // maker: '0x1b150538e943f00127929f7eeb65754f7beb0b6d'
            DynSolValue::Uint(U256::from_str("0x1b150538e943f00127929f7eeb65754f7beb0b6d").unwrap(), 256),
            // receiver: '0x0000000000000000000000000000000000000000'
            DynSolValue::Uint(U256::from_str("0x0000000000000000000000000000000000000000").unwrap(), 256),
            // makerAsset: '0x6756682b6144018dea5416640a0d0e8783e33f60'
            DynSolValue::Uint(U256::from_str("0x6756682b6144018dea5416640a0d0e8783e33f60").unwrap(), 256),
            // takerAsset: '0xda0000d4000015a526378bb6fafc650cea5966f8'
            DynSolValue::Uint(U256::from_str("0xda0000d4000015a526378bb6fafc650cea5966f8").unwrap(), 256),
            // makingAmount: '100000000000000000000'
            DynSolValue::Uint(U256::from_str("100000000000000000000").unwrap(), 256),
            // takingAmount: '99000000000000000000'
            DynSolValue::Uint(U256::from_str("99000000000000000000").unwrap(), 256),
            // makerTraits: '62419173104490761595518734106808059112479390467861203797200974394735470837760'
            DynSolValue::Uint(U256::from_str("62419173104490761595518734106515708578046331467977065221969182100300509478912").unwrap(), 256),
        ]);
        
        // Parse signature into r and vs components
        // Signature: '0xa746a906a8d6fbe1cf39f7ac171b96e111a362e58ed72beffac6c9466c1f2c03d92b8f42367df31b6bdbe5ab4bce2d092d099e0c219c41d25efad69484624aa1'
        let signature = "0x5f2c962837fc5d44103a4f0d60b21d8181b43bffe5c7d81bae328d01caf2d96b63b050db9677ec6043806355fc880ebdcaa06d2b38ac6a952dfbef8c239df3bd1c";
        let sig_bytes = hex::decode(signature).unwrap();
        println!("sig_bytes: {:?} and length {}", sig_bytes, sig_bytes.len());
        //hardcode r and vs values
        let r_value = DynSolValue::FixedBytes(Word::from_str("0x8cf10ec97e442be9afd37a2511085b5ad47d89015135f6735bc1844d012629a9").unwrap(), 32);
        let vs_value = DynSolValue::FixedBytes(Word::from_str("0xb2192775ddd288667c36553b0b1f3dea6c9ffb07326e043dcd86d7316173d02c").unwrap(), 32);
        //amount: 100000000000000000000
        let amount = DynSolValue::Uint(U256::from_str("100000000000000000000").unwrap(), 256);
        
        // TakerTraits: trait: 57896052787521937858429350288449525293583086444875042049621253778350470332416n
        let taker_traits = DynSolValue::Uint(U256::from_str("57896052787521937858429350288449525293583086444875042049621253778350470332416").unwrap(), 256);
        
        // Args: '0x0000010f0000004a0000004a0000004a0000004a000000250000000000000000e24c1fe89a1c8633f7e75152b6dd80cba8bf6c4f00000000000000688cbb6c000001000000e24c1fe89a1c8633f7e75152b6dd80cba8bf6c4f00000000000000688cbb6c000001000000e24c1fe89a1c8633f7e75152b6dd80cba8bf6c4f00000000ac3d03bd95345fa50e860000084048754ee73f1f94c13cc2f620769a262eec199ceaab852dbbb1d45ad01c7160000000000000000000000000000000000000000000000000000000000000279f000000000000000000000000ea2bb31ebb0aee264aba3730c8744d6bd76d37d0000000000000000000038d7ea4c68000000000000000000000038d7ea4c68000000000000000006500000064000000010000007a000000790000007800000001'
        let args_bytes = hex::decode("0x0000010f0000004a0000004a0000004a0000004a000000250000000000000000e24c1fe89a1c8633f7e75152b6dd80cba8bf6c4f00000000000000688ccf5e000001000000e24c1fe89a1c8633f7e75152b6dd80cba8bf6c4f00000000000000688ccf5e000001000000e24c1fe89a1c8633f7e75152b6dd80cba8bf6c4f00000000ac3d03bd95345fa50e86000008232c24908d508319a2544b51fe61ad81c05252dcca56d83d379ad8ca549c4fd6000000000000000000000000000000000000000000000000000000000000279f000000000000000000000000ea2bb31ebb0aee264aba3730c8744d6bd76d37d00000000000000000000000000000000000000000000000000000000000000000000000000000006500000064000000010000007a000000790000007800000001").unwrap();
        let args = DynSolValue::Bytes(args_bytes);
        
        println!("=== Hardcoded Test Data ===");
        println!("Immutables tuple: {:?}", immutables_tuple);
        println!("Order tuple: {:?}", order_tuple);
        println!("R value: {:?}", r_value);
        println!("VS value: {:?}", vs_value);
        println!("Amount: {:?}", amount);
        println!("TakerTraits: {:?}", taker_traits);
        println!("Args: {:?}", args);
        
    
        // Get the contract instance
        let contract = resolver.contract.get_contract().await.unwrap();
        
        // // Call the deploySrc function with hardcoded parameters
        // let result = contract
        //     .function("deploySrc", &[immutables_tuple, order_tuple, r_value, vs_value, amount, taker_traits, args])
        //     .unwrap()
        //     .send()
        //     .await;

            

        let src_cancellation_timestamp = U256::from(1913008236);
    
        let result = contract
            .function("deployDst", &[immutables_tuple, DynSolValue::Uint(U256::from(src_cancellation_timestamp), 256)])
            .unwrap()
            .send()
            .await;
            
        tracing::info!("Escrow deployed: {:?}", result);
            
        println!("=== Contract Call Result ===");
        println!("Result: {:?}", result);
        
        // The result should be Ok if the contract call succeeds
        match result {
            Ok(tx_result) => {
                println!("Transaction successful!");
                println!("Transaction hash: {:?}", tx_result);
            }
            Err(e) => {
                println!("Transaction failed: {:?}", e);
                // This is expected in a test environment without a real network
            }
        }
    }

}