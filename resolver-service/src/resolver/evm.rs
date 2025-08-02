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
        let signer = LocalSigner::from_str(&self.private_key)?;
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
    chain_id: u64,
}

#[async_trait::async_trait]
impl Resolver for EvmResolver {
    async fn deploy_dest_escrow(&self, order_action: &OrderAction) -> Result<()> {
        tracing::info!(
            chain_id=?self.chain_id,
            order_id=?order_action.order_id,
            "Deploying dest escrow"
        );
        


        let contract = self.contract.get_contract().await?;
        
        // For deployDst function, we need:
        // - dstImmutables (IBaseEscrow.Immutables) - tuple of 8 elements
        // - srcCancellationTimestamp (uint256)
        let secret_hash = order_action.order.secrets.first().map(|s| s.secret_hash.clone()).ok_or(anyhow::anyhow!("No secret hash found"))?;
        let safety_deposit = U256::from(0u64);
        // Create immutables tuple based on order data
        // IBaseEscrow.Immutables: (bytes32, bytes32, uint256, uint256, uint256, uint256, uint256, uint256)
        let making_amount_str = order_action.order.making_amount.to_plain_string();
        
        tracing::info!("order_action.order.order.taker_asset: {:?}", order_action.order.taker_asset);
        let immutables_tuple = DynSolValue::Tuple(vec![
            DynSolValue::FixedBytes(Word::from_str(&order_action.order.order_hash)?, 32), // orderHash (bytes32)
            DynSolValue::FixedBytes(Word::from_str(&secret_hash)?, 32), // hashlock (bytes32)
            DynSolValue::Uint(U256::from_str(&order_action.order.maker)?, 256), // maker (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.receiver)?, 256), // taker (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.taker_asset)?, 256), // token (uint256)
            DynSolValue::Uint(U256::from_str(&making_amount_str)?, 256), // amount (uint256)
            DynSolValue::Uint(safety_deposit, 256), // safetyDeposit (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.deadline.to_string())?, 256), // timelocks (uint256)
        ]);
        
        // Use current timestamp as srcCancellationTimestamp
        let src_cancellation_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| anyhow::anyhow!("Failed to get current timestamp"))?
            .as_secs();
        
    
        let src_cancellation_timestamp = U256::from(1913008236);
       

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
            "Deploying src escrow"
        );
        
        let contract = self.contract.get_contract().await?;
        
        
        let secret_hash = order_action.order.secrets.first().map(|s| s.secret_hash.clone()).ok_or(anyhow::anyhow!("No secret hash found"))?;
        
        
        let safety_deposit = U256::from(0u64);
        
        // IBaseEscrow.Immutables: (bytes32, bytes32, uint256, uint256, uint256, uint256, uint256, uint256)
        
        let making_amount_str = order_action.order.making_amount.to_plain_string();
        
        let immutables_tuple = DynSolValue::Tuple(vec![
            DynSolValue::FixedBytes(Word::from_str(&order_action.order.order_hash)?, 32), // orderHash (bytes32)
            DynSolValue::FixedBytes(Word::from_str(&secret_hash)?, 32), // hashlock (bytes32)
            DynSolValue::Uint(U256::from_str(&order_action.order.maker)?, 256), // maker (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.taker)?, 256), // taker (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.maker_asset)?, 256), // token (uint256)
            DynSolValue::Uint(U256::from_str(&making_amount_str)?, 256), // amount (uint256)
            DynSolValue::Uint(safety_deposit, 256), // safetyDeposit (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.timelock)?, 256), // timelocks (uint256)
        ]);
        
        // tracing::info!("immutables_tuple: {:#?}", immutables_tuple);
        
        // Create order tuple
        // IOrderMixin.Order: (uint256, uint256, uint256, uint256, uint256, uint256, uint256, uint256)
        // Convert BigDecimal amounts to strings, ensuring they fit in U256 range
        let taking_amount_str = order_action.order.taking_amount.to_plain_string();
        
        let making_amount_str = order_action.order.making_amount.to_plain_string();
        

        let taker_asset_hardcode = "0xda0000d4000015a526378bb6fafc650cea5966f8";

        let order_tuple = DynSolValue::Tuple(vec![
            DynSolValue::Uint(U256::from_str(&order_action.order.salt)?, 256), // salt (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.maker)?, 256), // maker (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.receiver)?, 256), // receiver (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.maker_asset)?, 256), // makerAsset (uint256)
            DynSolValue::Uint(U256::from_str(&taker_asset_hardcode)?, 256), // takerAsset (uint256)
            DynSolValue::Uint(U256::from_str(&making_amount_str)?, 256), // makingAmount (uint256)
            DynSolValue::Uint(U256::from_str(&taking_amount_str)?, 256), // takingAmount (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.maker_traits)?, 256), // makerTraits (uint256)
        ]);
        tracing::info!("order_tuple {:?}", order_tuple);
        tracing::info!("signature {:#?}", order_action.order.signature);

        let r_bytes = hex::decode(&order_action.order.signature["r"].as_str().unwrap())?;
        let vs_bytes = hex::decode(&order_action.order.signature["vs"].as_str().unwrap())?;

        
        let amt_str = if order_action.order.making_amount.to_string().contains('e') {
            let amt_str = order_action.order.making_amount.to_string();
            if let Some((mantissa, exponent)) = amt_str.split_once('e') {
                let exponent_value: i32 = exponent.parse()?;
                format!("{}{}", mantissa, "0".repeat(exponent_value as usize))
            } else {
                amt_str
            }
        } else {
            order_action.order.making_amount.to_string()
        };  

        // Use remaining maker amount as the fill amount
        let amount = U256::from_str(&amt_str)?;
        let taker_traits = U256::from_str(&order_action.order.taker_traits)?;
       
        
        // Use args from order action - convert from JSON to bytes
        let args_bytes = if let Some(args_str) = order_action.order.args.as_str() {
            hex::decode(args_str)?
        } else {
            vec![] // Default to empty bytes if args is not a string
        };
        let args = DynSolValue::Bytes(args_bytes);


        let result = contract
            .function("deploySrc", &[immutables_tuple, order_tuple, DynSolValue::FixedBytes(Word::from_slice(&r_bytes), 32), DynSolValue::FixedBytes(Word::from_slice(&vs_bytes), 32), DynSolValue::Uint(amount, 256), DynSolValue::Uint(taker_traits, 256), args])?
            .send()
            .await?;
            
        tracing::info!("deployed src escrow: {:?}", result);
        Ok(())
    }

    async fn widthdraw_src_escrow(&self, order_action: &OrderAction) -> Result<()> {
        tracing::info!(
            chain_id=?self.chain_id,
            order_id=?order_action.order_id,
            "Withdrawing src escrow"
        );
        
        let contract = self.contract.get_contract().await?;
        
        tracing::info!("order_action.order.secrets: {:?}", order_action.order.secrets);
        // Get the secret from the order
        let secret = order_action.order.secrets.first()
            .and_then(|s| s.secret.as_ref())
            .ok_or_else(|| anyhow::anyhow!("No secret found for withdrawal"))?;
        
        // Create immutables tuple for the withdraw call
        // IBaseEscrow.Immutables: (bytes32, bytes32, uint256, uint256, uint256, uint256, uint256, uint256)
        let secret_hash = order_action.order.secrets.first().map(|s| s.secret_hash.clone()).ok_or(anyhow::anyhow!("No secret hash found"))?;
        let safety_deposit = U256::from(0u64);
        let making_amount_str = order_action.order.making_amount.to_plain_string();
        
        let immutables_tuple = DynSolValue::Tuple(vec![
            DynSolValue::FixedBytes(Word::from_str(&order_action.order.order_hash)?, 32), // orderHash (bytes32)
            DynSolValue::FixedBytes(Word::from_str(&secret_hash)?, 32), // hashlock (bytes32)
            DynSolValue::Uint(U256::from_str(&order_action.order.maker)?, 256), // maker (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.taker)?, 256), // taker (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.maker_asset)?, 256), // token (uint256)
            DynSolValue::Uint(U256::from_str(&making_amount_str)?, 256), // amount (uint256)
            DynSolValue::Uint(safety_deposit, 256), // safetyDeposit (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.timelock)?, 256), // timelocks (uint256)
        ]);
        
            

        let source_escrow_address = order_action.order.src_escrow_address.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Source escrow address not found"))?;
        
        let result = contract
            .function("withdraw", &[
                DynSolValue::Address(Address::from_str(source_escrow_address)?), 
                DynSolValue::FixedBytes(Word::from_str(secret)?, 32), // secret (bytes32)
                immutables_tuple // immutables (tuple)
            ])?
            .send()
            .await?;
        
        tracing::info!("Withdraw transaction result: {:?}", result);
        Ok(())
    }

    async fn widthdraw_dest_escrow(&self, order_action: &OrderAction) -> Result<()> {
        tracing::info!(
            chain_id=?self.chain_id,
            order_id=?order_action.order_id,
            "Widthdrawing dest escrow"
        );
        
        let contract = self.contract.get_contract().await?;
        
        // Get the secret from the order
        let secret = order_action.order.secrets.first()
            .and_then(|s| s.secret.as_ref())
            .ok_or_else(|| anyhow::anyhow!("No secret found for withdrawal"))?;
        
        // Create immutables tuple for the withdraw call
        // IBaseEscrow.Immutables: (bytes32, bytes32, uint256, uint256, uint256, uint256, uint256, uint256)
        let secret_hash = order_action.order.secrets.first().map(|s| s.secret_hash.clone()).ok_or(anyhow::anyhow!("No secret hash found"))?;
        let safety_deposit = U256::from(0u64);
        let making_amount_str = order_action.order.making_amount.to_plain_string();
        
        let immutables_tuple = DynSolValue::Tuple(vec![
            DynSolValue::FixedBytes(Word::from_str(&order_action.order.order_hash)?, 32), // orderHash (bytes32)
            DynSolValue::FixedBytes(Word::from_str(&secret_hash)?, 32), // hashlock (bytes32)
            DynSolValue::Uint(U256::from_str(&order_action.order.maker)?, 256), // maker (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.taker)?, 256), // taker (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.taker_asset)?, 256), // token (uint256)
            DynSolValue::Uint(U256::from_str(&making_amount_str)?, 256), // amount (uint256)
            DynSolValue::Uint(safety_deposit, 256), // safetyDeposit (uint256)
            DynSolValue::Uint(U256::from_str(&order_action.order.timelock)?, 256), // timelocks (uint256)
        ]);
        
            

        let dst_escrow_address = order_action.order.dst_escrow_address.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Destination escrow address not found"))?;
        
        let result = contract
            .function("withdraw", &[
                DynSolValue::Address(Address::from_str(dst_escrow_address)?), 
                DynSolValue::FixedBytes(Word::from_str(secret)?, 32), // secret (bytes32)
                immutables_tuple // immutables (tuple)
            ])?
            .send()
            .await?;
        
        tracing::info!("Withdraw transaction result: {:?}", result);

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
            chain_id: chain_settings.chain_id,
        }
    }
}
