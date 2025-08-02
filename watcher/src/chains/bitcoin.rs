#[derive(Debug, Clone)]
pub struct Transaction {
    pub txid: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub block_height: Option<u32>,
    pub witness: Option<Vec<Vec<u8>>>,
}

#[derive(Debug, Clone)]
pub struct TxInput {
    pub prev_txid: String,
    pub vout: u32,
    pub script_sig: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct TxOutput {
    pub value: u64, // satoshis
    pub script_pubkey: Vec<u8>,
    pub address: Option<String>,
}

// pub struct BitcoinChain {
//     client: Arc<dyn BitcoinClient>,
//     _db: Arc<OrderbookProvider>,
// }
