use serde::{Deserialize, Serialize};

// Define the enum for HTLC types
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum HTLCType {
    P2tr2,  // p2tr with 2 spending path
    P2wsh2, // p2wsh with 2 spending path
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Bitcoin {
    pub initiator_pubkey: String, // No Option, use "" as default
    pub responder_pubkey: String, // No Option, use "" as default
    pub timelock: u64,
    pub amount: u64,
    pub htlc_type: HTLCType, // Required HTLC type for Bitcoin
    pub payment_hash: String, // Required payment hash
}

