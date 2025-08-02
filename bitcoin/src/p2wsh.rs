use std::str::FromStr;
use thiserror::Error;
use log::{debug, error, info};
use crate::tx_utils::{derive_keypair, build_input, build_output, build_transaction, compute_sighash, sign_ecdsa};
use crate::utils::Utxo;

use crate::swap::{HTLCType, Bitcoin};
use bitcoin::{
    PublicKey,
    opcodes,
    script::PushBytesBuf,
    secp256k1::{Secp256k1, Message},
    Address, Amount, KnownHrp, OutPoint, ScriptBuf, Transaction,
    TxOut, Txid, Witness,
};

#[derive(Error, Debug)]
pub enum HtlcError {
    #[error("Invalid payment hash")]
    InvalidPaymentHash,
    #[error("Invalid public key")]
    InvalidPublicKey,
    #[error("Push bytes buffer error")]
    PushBytesBufError,
    #[error("Invalid HTLC type")]
    InvalidHtlcType,
    #[error("Invalid private key")]
    InvalidPrivateKey,
    #[error("Invalid Txid: {0}")]
    InvalidTxid(String),
    #[error("Failed to compute sighash for input {index}")]
    SighashError { index: usize },
}

fn generate_p2wsh_address(
    bitcoin: &Bitcoin,
    network: KnownHrp,
) -> Result<Address, HtlcError> {
    debug!("Generating P2WSH address for network: {:?}", network);
    
    if HTLCType::P2wsh2 != bitcoin.htlc_type {
        error!("Invalid HTLC type: {:?}", bitcoin.htlc_type);
        return Err(HtlcError::InvalidHtlcType);
    }

    let script_buf = p2wsh_htlc_script(
        &bitcoin.payment_hash,
        &bitcoin.initiator_pubkey,
        &bitcoin.responder_pubkey,
        bitcoin.timelock,
    )?;

    let address = Address::p2wsh(
        &script_buf,
        network,
    );

    debug!("P2WSH address generated successfully: {}", address);
    Ok(address)
}

pub fn p2wsh_htlc_script(
    payment_hash: &str,
    initiator_pubkey: &str,
    responder_pubkey: &str,
    timelock: u64,
) -> Result<ScriptBuf, HtlcError> {
    debug!("Building P2WSH HTLC script with timelock: {}", timelock);

    // Decode payment hash from hex
    let payment_hash_bytes = hex::decode(payment_hash)
        .map_err(|_| {
            error!("Failed to decode payment hash");
            HtlcError::InvalidPaymentHash
        })?;
    let payment_hash_buf = PushBytesBuf::try_from(payment_hash_bytes)
        .map_err(|_| {
            error!("Failed to create push bytes buffer");
            HtlcError::PushBytesBufError
        })?;

    // Parse public keys
    let initiator_pubkey = PublicKey::from_str(initiator_pubkey)
        .map_err(|_| {
            error!("Failed to parse initiator public key");
            HtlcError::InvalidPublicKey
        })?;
    let responder_pubkey = PublicKey::from_str(responder_pubkey)
        .map_err(|_| {
            error!("Failed to parse responder public key");
            HtlcError::InvalidPublicKey
        })?;

    // Build the HTLC script
    debug!("Constructing HTLC script");
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

    debug!("HTLC script constructed successfully");
    Ok(htlc_script)
}

pub fn redeem_p2wsh_htlc(
    bitcoin: &Bitcoin,
    preimage: &str,
    receiver_private_key: &str,
    utxos: Vec<Utxo>,
    transfer_to_address: &Address,
    fee_rate_per_vb: u64,
    network: KnownHrp,
) -> Result<Transaction, HtlcError> {
    let secp = Secp256k1::new();
    info!("Starting P2WSH redeem for bitcoin: {:?}", bitcoin);

    // 1️⃣ Generate P2WSH address
    let htlc_address = generate_p2wsh_address(bitcoin, network)?;

    // 2️⃣ Get the HTLC witness script
    let witness_script = p2wsh_htlc_script(
        &bitcoin.payment_hash,
        &bitcoin.initiator_pubkey,
        &bitcoin.responder_pubkey,
        bitcoin.timelock,
    )?;

    // 3️⃣ Derive receiver's keypair
    let keypair = derive_keypair(receiver_private_key)
        .map_err(|_| {
            error!("Failed to derive receiver keypair");
            HtlcError::InvalidPrivateKey
        })?;

    // 4️⃣ Prepare inputs, prevouts, and total input amount
    let mut inputs = Vec::new();
    let mut prevouts = Vec::new();
    let mut total_amount = Amount::from_sat(0);

    for utxo in &utxos {
        let prev_txid = Txid::from_str(&utxo.txid)
            .map_err(|e| {
                error!("Failed to parse txid: {}", utxo.txid);
                HtlcError::InvalidTxid(e.to_string())
            })?;
        let outpoint = OutPoint::new(prev_txid, utxo.vout);
        let input = build_input(outpoint, None);
        inputs.push(input);

        let amount = Amount::from_sat(utxo.value);
        total_amount += amount;

        let prevout = TxOut {
            value: amount,
            script_pubkey: htlc_address.script_pubkey(),
        };
        prevouts.push(prevout);
    }

    let input_count = inputs.len();
    let output_count = 1;

    // 5️⃣ Estimate fees
    let witness_size_per_input = 1 + 73 + 32 + 1 + witness_script.to_bytes().len(); // Sig + Preimage + Script push + OP_1 (true branch)
    let fee = estimate_htlc_fee(
        input_count,
        output_count,
        witness_size_per_input,
        fee_rate_per_vb,
    );

    // 6️⃣ Build output
    let output = build_output(total_amount - fee, transfer_to_address);

    // 7️⃣ Build unsigned transaction
    let mut tx = build_transaction(inputs, vec![output]);

    // 8️⃣ Prepare shared data
    let preimage_bytes = hex::decode(preimage)
        .map_err(|_| {
            error!("Failed to decode preimage");
            HtlcError::InvalidPaymentHash
        })?;

    // 🔄 Sign each input individually and assign witness
    for i in 0..tx.input.len() {
        let sighash = compute_sighash(&tx, i, &prevouts, &witness_script)
            .map_err(|_| HtlcError::SighashError { index: i })?;

        let msg = Message::from_digest_slice(&sighash)
            .map_err(|_| HtlcError::SighashError { index: i })?;

        let signature = sign_ecdsa(&secp, &msg, &keypair);

        // P2WSH witness stack for redeem path: <signature> <preimage> <1> <witness_script>
        // The <1> pushes the IF branch (true) to execute the redeem path
        let mut witness = Witness::new();
        witness.push(signature.to_vec()); // signature with sighash type
        witness.push(preimage_bytes.clone()); // preimage
        witness.push([1u8]); // OP_1 to take the IF branch
        witness.push(witness_script.to_bytes()); // witness script

        tx.input[i].witness = witness;
    }

    info!("Redeemed P2WSH transaction: {:?}", tx);
    Ok(tx)
}

pub fn refund_p2wsh_htlc(
    bitcoin: &Bitcoin,
    sender_private_key: &str,
    utxos: Vec<Utxo>,
    refund_to_address: &Address,
    fee_rate_per_vb: u64,
    network: KnownHrp,
) -> Result<Transaction, HtlcError> {
    let secp = Secp256k1::new();
    info!("Starting P2WSH refund for bitcoin: {:?}", bitcoin);

    // 1️⃣ Generate P2WSH address
    let htlc_address = generate_p2wsh_address(bitcoin, network)?;

    // 2️⃣ Get the HTLC witness script
    let witness_script = p2wsh_htlc_script(
        &bitcoin.payment_hash,
        &bitcoin.initiator_pubkey,
        &bitcoin.responder_pubkey,
        bitcoin.timelock,
    )?;

    // 3️⃣ Derive sender's keypair
    let keypair = derive_keypair(sender_private_key)
        .map_err(|_| {
            error!("Failed to derive sender keypair");
            HtlcError::InvalidPrivateKey
        })?;

    // 4️⃣ Prepare inputs, prevouts, and total input amount
    let mut inputs = Vec::new();
    let mut prevouts = Vec::new();
    let mut total_amount = Amount::from_sat(0);

    for utxo in &utxos {
        let prev_txid = Txid::from_str(&utxo.txid)
            .map_err(|e| {
                error!("Failed to parse txid: {}", utxo.txid);
                HtlcError::InvalidTxid(e.to_string())
            })?;
        let outpoint = OutPoint::new(prev_txid, utxo.vout);
        // Set sequence to timelock for CSV
        let input = build_input(outpoint, Some(bitcoin.timelock as u32));
        inputs.push(input);

        let amount = Amount::from_sat(utxo.value);
        total_amount += amount;

        let prevout = TxOut {
            value: amount,
            script_pubkey: htlc_address.script_pubkey(),
        };
        prevouts.push(prevout);
    }

    let input_count = inputs.len();
    let output_count = 1;

    // 5️⃣ Estimate fees
    let witness_size_per_input = 1 + 73 + 1 + witness_script.to_bytes().len(); // Sig + Script push + OP_0 (false branch)
    let fee = estimate_htlc_fee(
        input_count,
        output_count,
        witness_size_per_input,
        fee_rate_per_vb,
    );

    // 6️⃣ Build output
    let output = build_output(total_amount - fee, refund_to_address);

    // 7️⃣ Build unsigned transaction
    let mut tx = build_transaction(inputs, vec![output]);

    // 🔄 Sign each input individually and assign witness
    for i in 0..tx.input.len() {
        let sighash = compute_sighash(&tx, i, &prevouts, &witness_script)
            .map_err(|_| HtlcError::SighashError { index: i })?;

        let msg = Message::from_digest_slice(&sighash)
            .map_err(|_| HtlcError::SighashError { index: i })?;

        let signature = sign_ecdsa(&secp, &msg, &keypair);

        // P2WSH witness stack for refund path: <signature> <0> <witness_script>
        // The <0> pushes the ELSE branch (false) to execute the refund path
        let mut witness = Witness::new();
        witness.push(signature.to_vec()); // signature with sighash type
        witness.push([]); // OP_0 to take the ELSE branch
        witness.push(witness_script.to_bytes()); // witness script

        tx.input[i].witness = witness;
    }

    info!("Refunded P2WSH transaction: {:?}", tx);
    Ok(tx)
}

fn estimate_htlc_fee(
    input_count: usize,
    output_count: usize,
    witness_size_per_input: usize,
    fee_rate_per_vb: u64,
) -> Amount {
    let base_size = 6 + (input_count * 40) + 1 + (output_count * 43) + 4;
    let total_witness_size = input_count * witness_size_per_input;
    let total_weight = base_size * 4 + total_witness_size;
    let vsize = (total_weight + 3) / 4;
    Amount::from_sat(vsize as u64 * fee_rate_per_vb)
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;
    use crate::utils::{UtxoStatus};

    // Expected P2WSH address for the test case
    const TEST_EXPECTED_ADDRESS: &str = "tb1qvcdnft8sszsjrfy0k6dw8t3qkf76au6j7axycgy0qtwdyvtvn2rsumwnly";

    // Helper to initialize logger
    fn init_logger() {
        let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .try_init();
    }

    // Helper to create a mock Bitcoin struct
    fn create_mock_bitcoin() -> Bitcoin {
        Bitcoin {
            initiator_pubkey: "0280b2aa1b37d358607896a0747f6104d576fd1b887792e3b2fdc37c7170a8a4d7".to_string(),
            responder_pubkey: "03d168e6449eae4d673b0020c7e7cbf0b4ba11fddf762450a1cce444b8206d3e0f".to_string(),
            timelock: 144,
            amount: 10000,
            htlc_type: HTLCType::P2wsh2,
            payment_hash: "c3a704c5669f96c853fd03521e2318f784e1fe743568fdea9fe3eca2850b3368".to_string(),
        }
    }

    fn create_mock_utxo(block_height: u32, txid: &str, vout: u32, value: u64) -> Utxo {
        Utxo {
            txid: txid.to_string(),
            vout,
            value,
            status: UtxoStatus {
                confirmed: true,
                block_height: block_height,
                block_hash: "0000000000000000000000000000000000000000000000000000000000000000"
                    .to_string(),
                block_time: 1234567890,
            },
        }
    }

    #[test]
    fn test_generate_p2wsh_address_success() {
        init_logger();
        let bitcoin = create_mock_bitcoin();
        let network = KnownHrp::Testnets;

        let result = generate_p2wsh_address(&bitcoin, network);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let address = result.unwrap();
        assert_eq!(address.to_string(), TEST_EXPECTED_ADDRESS, "Generated address does not match expected");
    }

    #[test]
    fn test_p2wsh_htlc_script_creation() {
        init_logger();
        let bitcoin = create_mock_bitcoin();

        let result = p2wsh_htlc_script(
            &bitcoin.payment_hash,
            &bitcoin.initiator_pubkey,
            &bitcoin.responder_pubkey,
            bitcoin.timelock,
        );
        
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let script = result.unwrap();
        assert!(!script.is_empty(), "Script should not be empty");
    }

     #[test]
    fn test_redeem_taproot_htlc_success() {
        init_logger();
        let bitcoin = create_mock_bitcoin();
        let preimage = "1572a86fb4b1f15623da10e34034fd151090d37e6f0f3ef4f69926f7f3388b78";
        let private_key = "b883a78959fadb3c31036b724be10dd08cec325f2e82812e9e0291ab0863ab84";

        let network = KnownHrp::Testnets;
        let htlc_address = generate_p2wsh_address(&bitcoin, network);
        assert!(htlc_address.is_ok(), "Expected Ok, got {:?}", htlc_address);
        let htlc_address = htlc_address.unwrap();

        println!("HTLC Address: {}", htlc_address);

        let transfer_to_address = Address::from_str("tb1q7rg6er2dtafjm9y6kemjqh3a932a6rlwrl9l4v")
            .unwrap()
            .assume_checked();

        let utxo = create_mock_utxo(
            2315994,
            "3dae1de0ab840ebc5f1b27ddc275acf52e7c86117218157986504ac8eaac98e1",
            0,
            1000,
        );
        let utxos = vec![utxo];
        let fee_rate_per_vb = 3;
        let result = redeem_p2wsh_htlc(
            &bitcoin,
            preimage,
            private_key,
            utxos,
            &transfer_to_address,
            fee_rate_per_vb,
            network,
        );

        let tx = result.expect("Expected Ok, got Err");

        let tx_hex = bitcoin::consensus::encode::serialize_hex(&tx);
        info!("Redeemed transaction hex: {}", tx_hex);

        assert_eq!(tx_hex, "02000000000101e198aceac84a50867915187211867c2ef5ac75c2dd271b5fbc0e84abe01dae3d0000000000fdffffff012902000000000000160014f0d1ac8d4d5f532d949ab677205e3d2c55dd0fee04483045022100ecbb73757962ea1f11425cf1c61d21c6d2ef65e2d793a4bb5f2023089443d8cd02201f7175abeb14110532f36ec5a359f60a90a12fc0042185b55fb45d46afc80fe901201572a86fb4b1f15623da10e34034fd151090d37e6f0f3ef4f69926f7f3388b7801017163a820c3a704c5669f96c853fd03521e2318f784e1fe743568fdea9fe3eca2850b3368882103d168e6449eae4d673b0020c7e7cbf0b4ba11fddf762450a1cce444b8206d3e0fac67029000b275210280b2aa1b37d358607896a0747f6104d576fd1b887792e3b2fdc37c7170a8a4d7ac6800000000");
    }

     #[test]
    fn test_refund_taproot_htlc_success() {
        init_logger();
        let mut bitcoin = create_mock_bitcoin();
        bitcoin.payment_hash =
            "c3a704c5669f96c853fd03521e2318f784e1fe743568fdea9fe3eca2850b3368".to_string();
        bitcoin.timelock = 5;
        let private_key = "0bb90fe46bc4145c6e3c33dd08918eb213a0346e3b77ce0e6cffb3684b3de2f7";
        let network = KnownHrp::Testnets;
        let htlc_address = generate_p2wsh_address(&bitcoin, network);
        assert!(htlc_address.is_ok(), "Expected Ok, got {:?}", htlc_address);
        let htlc_address = htlc_address.unwrap();
        print!("HTLC Address: {}", htlc_address);

        let utxo = create_mock_utxo(
            2315994,
            "1f93459a31c5cdaf86daff892b29343aca2e85f7bd27761ab155df23423b8223",
            0,
            1000,
        );
        let utxos = vec![utxo];
        let fee_rate_per_vb = 3;

        let refund_to_address = Address::from_str("tb1qmrmpwhh79ayxmym8rg7ncg4ttw2c7c8mjrqean")
            .unwrap()
            .assume_checked();

        let result = refund_p2wsh_htlc(
            &bitcoin,
            private_key,
            utxos,
            &refund_to_address,
            fee_rate_per_vb,
            network,
        );

        let tx = result.expect("Expected Ok, got Err");
        let tx_hex = bitcoin::consensus::encode::serialize_hex(&tx);
        info!("Refunded transaction hex: {}", tx_hex);
        assert_eq!(tx_hex, "0200000000010123823b4223df55b11a7627bdf7852eca3a34292b89ffda86afcdc5319a45931f000000000005000000014102000000000000160014d8f6175efe2f486d93671a3d3c22ab5b958f60fb03483045022100bde1b81b52e5f2ad2fd879f53405e48d3c3bd0ad35b66d8bb2fcfc5c579cea5c022058b3f6e50396ae9d5864b625c45ed9757757d23887ee3a901e56b1dfb4319a7101006f63a820c3a704c5669f96c853fd03521e2318f784e1fe743568fdea9fe3eca2850b3368882103d168e6449eae4d673b0020c7e7cbf0b4ba11fddf762450a1cce444b8206d3e0fac6755b275210280b2aa1b37d358607896a0747f6104d576fd1b887792e3b2fdc37c7170a8a4d7ac6800000000");
    }

   
}