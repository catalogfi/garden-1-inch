use candid::CandidType;
use ic_cdk::{query, update};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::{common::DerivationPath, ecdsa::get_ecdsa_public_key, BTC_CONTEXT};
use bitcoin::{Address, CompressedPublicKey, PublicKey, ScriptBuf, opcodes};
use bitcoin::script::PushBytesBuf;
use std::str::FromStr;
use crate::{
    common::{get_fee_per_byte},
    ecdsa::{sign_with_ecdsa},
    p2wpkh,
};
use ic_cdk::bitcoin_canister::{
    bitcoin_get_utxos, bitcoin_send_transaction, GetUtxosRequest, SendTransactionRequest,
    bitcoin_get_balance, GetBalanceRequest,
};
use bitcoin::consensus::serialize;

#[derive(CandidType, Clone)]
pub struct OrderDetail {
    pub initiator_pubkey: String,
    pub time_lock: u64,
    pub secret_hash: String,
    pub order_address: Option<String>, // P2WPKH address for this order
}

#[derive(CandidType, Clone)]
pub struct OrderWithdrawInfo {
    pub order_address: String,          // The order's P2WPKH address (source)
    pub order_balance: u64,             // Balance of the order address in satoshis
    pub htlc_address: String,           // The P2WSH HTLC address (destination)
    pub order_details: OrderDetail,     // The order details
}

#[derive(CandidType, Clone)]
struct OrderStorage {
    orders: HashMap<u64, OrderDetail>,
    next_order_no: u64,
}

impl OrderStorage {
    fn new() -> Self {
        Self {
            orders: HashMap::new(),
            next_order_no: 1,
        }
    }
}

thread_local! {
    static STORAGE: RefCell<OrderStorage> = RefCell::new(OrderStorage::new());
}

/// Creates a new order and returns the order number
#[update]
pub fn create_order(initiator_pubkey: String, time_lock: u64, secret_hash: String) -> u64 {
    STORAGE.with(|s| {
        let mut storage = s.borrow_mut();
        let order_no = storage.next_order_no;
        
        let order_detail = OrderDetail {
            initiator_pubkey,
            time_lock,
            secret_hash,
            order_address: None, // Address will be generated separately
        };
        
        storage.orders.insert(order_no, order_detail);
        storage.next_order_no += 1;
        
        order_no
    })
}

/// Retrieves a specific order by order number
#[query]
pub fn get_order(order_no: u64) -> Option<OrderDetail> {
    STORAGE.with(|s| {
        s.borrow().orders.get(&order_no).cloned()
    })
}

/// Retrieves all orders
#[query]
pub fn get_all_orders() -> Vec<(u64, OrderDetail)> {
    STORAGE.with(|s| {
        s.borrow().orders.iter().map(|(k, v)| (*k, v.clone())).collect()
    })
}

/// Gets the next order number that will be assigned
#[query]
pub fn get_next_order_no() -> u64 {
    STORAGE.with(|s| {
        s.borrow().next_order_no
    })
}

/// Creates a P2WPKH address for a specific order and stores it
/// Uses the order number as account number for unique derivation paths
#[update]
pub async fn get_order_address(order_no: u64) -> Result<String, String> {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());
    
    // Check if the order exists
    let order_exists = STORAGE.with(|s| {
        s.borrow().orders.contains_key(&order_no)
    });
    
    if !order_exists {
        return Err(format!("Order {} does not exist", order_no));
    }
    
    // Check if address already exists for this order
    let existing_address = STORAGE.with(|s| {
        s.borrow().orders.get(&order_no).and_then(|order| order.order_address.clone())
    });
    
    if let Some(address) = existing_address {
        return Ok(address);
    }
    
    // Use order number as account number for unique derivation path
    // This ensures each order has a unique address
    let derivation_path = DerivationPath::p2wpkh(order_no as u32, 0);
    
    // Get the ECDSA public key for this specific derivation path
    let public_key = get_ecdsa_public_key(&ctx, derivation_path.to_vec_u8_path()).await;
    
    // Create a CompressedPublicKey from the raw public key bytes
    let public_key = CompressedPublicKey::from_slice(&public_key)
        .map_err(|e| format!("Failed to create public key: {}", e))?;
    
    // Generate a P2WPKH Bech32 address
    let address = Address::p2wpkh(&public_key, ctx.bitcoin_network).to_string();
    
    // Store the address in the order
    STORAGE.with(|s| {
        let mut storage = s.borrow_mut();
        if let Some(order) = storage.orders.get_mut(&order_no) {
            order.order_address = Some(address.clone());
        }
    });
    
    Ok(address)
}

/// Generates a P2WSH HTLC script
fn generate_htlc_script(
    payment_hash: &str,
    initiator_pubkey: &str,
    responder_pubkey: &str,
    timelock: u64,
) -> Result<ScriptBuf, String> {
    // Decode payment hash from hex
    let payment_hash_bytes = hex::decode(payment_hash)
        .map_err(|_| "Failed to decode payment hash".to_string())?;
    
    // Convert bytes to PushBytesBuf
    let mut payment_hash_buf = PushBytesBuf::new();
    for byte in payment_hash_bytes {
        payment_hash_buf.push(byte).map_err(|_| "Failed to push byte to buffer".to_string())?;
    }

    // Parse public keys
    let initiator_pubkey = PublicKey::from_str(initiator_pubkey)
        .map_err(|_| "Failed to parse initiator public key".to_string())?;
    let responder_pubkey = PublicKey::from_str(responder_pubkey)
        .map_err(|_| "Failed to parse responder public key".to_string())?;

    // Build the HTLC script
    let htlc_script = ScriptBuf::builder()
        .push_opcode(opcodes::all::OP_IF)
        .push_opcode(opcodes::all::OP_SHA256)
        .push_slice(&payment_hash_buf)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_key(&responder_pubkey)
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .push_opcode(opcodes::all::OP_ELSE)
        .push_int(timelock as i64)
        .push_opcode(opcodes::all::OP_CSV)
        .push_opcode(opcodes::all::OP_DROP)
        .push_key(&initiator_pubkey)
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .push_opcode(opcodes::all::OP_ENDIF)
        .into_script();

    Ok(htlc_script)
}

/// Generates a P2WSH address for HTLC
fn generate_htlc_address(
    payment_hash: &str,
    initiator_pubkey: &str,
    responder_pubkey: &str,
    timelock: u64,
    network: bitcoin::Network,
) -> Result<Address, String> {
    let script_buf = generate_htlc_script(
        payment_hash,
        initiator_pubkey,
        responder_pubkey,
        timelock,
    )?;

    let address = Address::p2wsh(&script_buf, network);
    Ok(address)
}

/// Reads order withdrawal information without executing the transaction
/// Returns order address, balance, HTLC address, and order details
#[update]
pub async fn preview_order_withdrawal(order_no: u64, responder_pubkey: String) -> Result<OrderWithdrawInfo, String> {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    // Get the order details
    let order = STORAGE.with(|s| {
        s.borrow().orders.get(&order_no).cloned()
    });

    let order = match order {
        Some(order) => order,
        None => return Err(format!("Order {} does not exist", order_no)),
    };

    // Validate responder public key
    PublicKey::from_str(&responder_pubkey)
        .map_err(|_| "Invalid responder public key".to_string())?;

    // Generate P2WSH HTLC address
    let htlc_address = generate_htlc_address(
        &order.secret_hash,
        &order.initiator_pubkey,
        &responder_pubkey,
        order.time_lock,
        ctx.bitcoin_network,
    )?;

    // Get the P2WPKH address for this order
    let derivation_path = DerivationPath::p2wpkh(order_no as u32, 0);
    let own_public_key = get_ecdsa_public_key(&ctx, derivation_path.to_vec_u8_path()).await;
    let own_compressed_public_key = CompressedPublicKey::from_slice(&own_public_key)
        .map_err(|e| format!("Failed to create public key: {}", e))?;
    let order_address = Address::p2wpkh(&own_compressed_public_key, ctx.bitcoin_network);

    // Get balance of the order address
    let order_balance = bitcoin_get_balance(&GetBalanceRequest {
        address: order_address.to_string(),
        network: ctx.network,
        min_confirmations: None,
    })
    .await
    .map_err(|e| format!("Failed to get balance: {:?}", e))?;

    Ok(OrderWithdrawInfo {
        order_address: order_address.to_string(),
        order_balance,
        htlc_address: htlc_address.to_string(),
        order_details: order,
    })
}

/// Executes order withdrawal by sending funds from order address to HTLC address
/// Takes order number, responder pubkey, and amount
#[update]
pub async fn execute_order_withdraw_to_htlc(order_no: u64, responder_pubkey: String, amount_in_satoshi: u64) -> Result<String, String> {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    if amount_in_satoshi == 0 {
        return Err("Amount must be greater than 0".to_string());
    }

    // Get the order details
    let order = STORAGE.with(|s| {
        s.borrow().orders.get(&order_no).cloned()
    });

    let order = match order {
        Some(order) => order,
        None => return Err(format!("Order {} does not exist", order_no)),
    };

    // Validate responder public key
    PublicKey::from_str(&responder_pubkey)
        .map_err(|_| "Invalid responder public key".to_string())?;

    // Generate P2WSH HTLC address
    let htlc_address = generate_htlc_address(
        &order.secret_hash,
        &order.initiator_pubkey,
        &responder_pubkey,
        order.time_lock,
        ctx.bitcoin_network,
    )?;

    // Get the P2WPKH address for this order (source address)
    let derivation_path = DerivationPath::p2wpkh(order_no as u32, 0);
    let own_public_key = get_ecdsa_public_key(&ctx, derivation_path.to_vec_u8_path()).await;
    let own_compressed_public_key = CompressedPublicKey::from_slice(&own_public_key)
        .map_err(|e| format!("Failed to create public key: {}", e))?;
    let own_public_key = PublicKey::from_slice(&own_public_key)
        .map_err(|e| format!("Failed to create public key: {}", e))?;
    let own_address = Address::p2wpkh(&own_compressed_public_key, ctx.bitcoin_network);

    // Get UTXOs from the order's P2WPKH address
    let own_utxos = bitcoin_get_utxos(&GetUtxosRequest {
        address: own_address.to_string(),
        network: ctx.network,
        filter: None,
    })
    .await
    .map_err(|e| format!("Failed to get UTXOs: {:?}", e))?
    .utxos;

    if own_utxos.is_empty() {
        return Err("No UTXOs available for this order".to_string());
    }

    // Check if there's enough balance before proceeding
    let account_balance = bitcoin_get_balance(&GetBalanceRequest {
        address: own_address.to_string(),
        network: ctx.network,
        min_confirmations: None,
    })
    .await
    .map_err(|e| format!("Failed to get balance: {:?}", e))?;

    if account_balance < amount_in_satoshi {
        return Err(format!(
            "Insufficient balance: {} satoshis available, but {} satoshis requested", 
            account_balance, 
            amount_in_satoshi
        ));
    }

    // Build the transaction that sends `amount` to the HTLC address
    let fee_per_byte = get_fee_per_byte(&ctx).await;
    let (transaction, prevouts) = p2wpkh::build_transaction(
        &ctx,
        &own_public_key,
        &own_address,
        &own_utxos,
        &htlc_address,
        amount_in_satoshi,
        fee_per_byte,
    )
    .await;

    // Sign the transaction
    let signed_transaction = p2wpkh::sign_transaction(
        &ctx,
        &own_public_key,
        &own_address,
        transaction,
        &prevouts,
        derivation_path.to_vec_u8_path(),
        sign_with_ecdsa,
    )
    .await;

    // Send the transaction to the Bitcoin network
    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network,
        transaction: serialize(&signed_transaction),
    })
    .await
    .map_err(|e| format!("Failed to send transaction: {:?}", e))?;

    // Return the transaction ID
    Ok(signed_transaction.compute_txid().to_string())
}
