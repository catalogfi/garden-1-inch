use crate::{
    chains::traits::Chain, orderbook::provider::OrderbookProvider, types::WatcherEventType,
};
use alloy::{
    dyn_abi::{DecodedEvent, DynSolValue, EventExt},
    hex,
    json_abi::JsonAbi,
    network::AnyNetwork,
    primitives::{Address, B256, LogData, U256},
    providers::{Provider as EthereumProvider, RootProvider},
    rpc::{
        client::RpcClient,
        types::{Filter, Log},
    },
    transports::http::Http,
};
use async_trait::async_trait;
use serde_json::Value;
use std::{fs, path::Path, str::FromStr, sync::Arc, time::Duration};
use tokio::time::sleep;
use tracing::{error, info};

const MAX_BLOCK_SPAN: u64 = 200;
const POLLING_INTERVAL: u64 = 5;

#[derive(Debug, Clone)]
pub struct ParamOneTuple {
    pub order_hash: String,
    pub hashlock: String,
    pub maker_address: String,
    pub taker_address: String,
    pub token_address: String,
    pub src_amount: U256,
    pub safety_deposit: U256,
    pub timelock: U256,
}

#[derive(Debug, Clone)]
pub struct ParamTwoTuple {
    pub maker_address: String,
    pub taker_amount: U256,
    pub destination_token_address: String,
    pub safety_deposit: U256,
    pub chain_id: U256,
}

#[derive(Debug, Clone)]
pub struct ParamThreeTuple {
    pub escrow_address: String,
}

#[derive(Debug, Clone)]
pub struct SrcEscrowCreatedEvent {
    pub param_one: ParamOneTuple,
    pub param_two: ParamTwoTuple,
    pub param_three: ParamThreeTuple,
}

#[derive(Debug)]
pub struct EthereumChain {
    client: Arc<RootProvider<AnyNetwork>>,
    contract_address: String,
    db: Arc<OrderbookProvider>,
    last_block: Option<u64>,
    start_block: u64,
}

impl ParamOneTuple {
    fn from_tuple_fields(fields: &[DynSolValue]) -> anyhow::Result<Self> {
        if fields.len() != 8 {
            return Err(anyhow::anyhow!(
                "Expected 8 fields in param_one tuple, got {}",
                fields.len()
            ));
        }

        let order_hash = match &fields[0] {
            DynSolValue::FixedBytes(bytes, _) => hex::encode(bytes),
            _ => return Err(anyhow::anyhow!("Field 0 should be FixedBytes")),
        };

        let hashlock = match &fields[1] {
            DynSolValue::FixedBytes(bytes, _) => hex::encode(bytes),
            _ => return Err(anyhow::anyhow!("Field 1 should be FixedBytes")),
        };

        let maker_address = match &fields[2] {
            DynSolValue::Uint(value, _) => {
                let addr_bytes = value.to_be_bytes::<32>();
                let addr = Address::from_slice(&addr_bytes[12..]);
                format!("0x{}", hex::encode(addr))
            }
            _ => return Err(anyhow::anyhow!("Field 2 should be Uint (address)")),
        };

        let taker_address = match &fields[3] {
            DynSolValue::Uint(value, _) => {
                let addr_bytes = value.to_be_bytes::<32>();
                let addr = Address::from_slice(&addr_bytes[12..]);
                format!("0x{}", hex::encode(addr))
            }
            _ => return Err(anyhow::anyhow!("Field 3 should be Uint (address)")),
        };

        let token_address = match &fields[4] {
            DynSolValue::Uint(value, _) => {
                let addr_bytes = value.to_be_bytes::<32>();
                let addr = Address::from_slice(&addr_bytes[12..]);
                format!("0x{}", hex::encode(addr))
            }
            _ => return Err(anyhow::anyhow!("Field 4 should be Uint (address)")),
        };

        let src_amount = match &fields[5] {
            DynSolValue::Uint(value, _) => *value,
            _ => return Err(anyhow::anyhow!("Field 5 should be Uint")),
        };

        let safety_deposit = match &fields[6] {
            DynSolValue::Uint(value, _) => *value,
            _ => return Err(anyhow::anyhow!("Field 6 should be Uint")),
        };

        let timelock = match &fields[7] {
            DynSolValue::Uint(value, _) => *value,
            _ => return Err(anyhow::anyhow!("Field 7 should be Uint")),
        };

        Ok(ParamOneTuple {
            order_hash,
            hashlock,
            maker_address,
            taker_address,
            token_address,
            src_amount,
            safety_deposit,
            timelock,
        })
    }
}

impl ParamTwoTuple {
    fn from_tuple_fields(fields: &[DynSolValue]) -> anyhow::Result<Self> {
        if fields.len() != 5 {
            return Err(anyhow::anyhow!(
                "Expected 5 fields in param_two tuple, got {}",
                fields.len()
            ));
        }

        let maker_address = match &fields[0] {
            DynSolValue::Uint(value, _) => {
                let addr_bytes = value.to_be_bytes::<32>();
                let addr = Address::from_slice(&addr_bytes[12..]);
                format!("0x{}", hex::encode(addr))
            }
            _ => return Err(anyhow::anyhow!("Field 0 should be Uint (address)")),
        };

        let taker_amount = match &fields[1] {
            DynSolValue::Uint(value, _) => *value,
            _ => return Err(anyhow::anyhow!("Field 1 should be Uint")),
        };

        let destination_token_address = match &fields[2] {
            DynSolValue::Uint(value, _) => {
                let addr_bytes = value.to_be_bytes::<32>();
                let addr = Address::from_slice(&addr_bytes[12..]);
                format!("0x{}", hex::encode(addr))
            }
            _ => return Err(anyhow::anyhow!("Field 2 should be Uint (address)")),
        };

        let safety_deposit = match &fields[3] {
            DynSolValue::Uint(value, _) => *value,
            _ => return Err(anyhow::anyhow!("Field 3 should be Uint")),
        };

        let chain_id = match &fields[4] {
            DynSolValue::Uint(value, _) => *value,
            _ => return Err(anyhow::anyhow!("Field 4 should be Uint")),
        };

        Ok(ParamTwoTuple {
            maker_address,
            taker_amount,
            destination_token_address,
            safety_deposit,
            chain_id,
        })
    }
}

#[async_trait]
impl Chain for EthereumChain {
    type Event = Log;

    async fn new(
        rpc_url: String,
        contract_address: String,
        db: Arc<OrderbookProvider>,
        start_block: u64,
    ) -> anyhow::Result<Self> {
        let transport = Http::new(rpc_url.parse()?);
        let provider = RootProvider::new(RpcClient::new(transport, false));
        let client = Arc::new(provider);

        Ok(Self {
            client,
            contract_address,
            db,
            last_block: None,
            start_block,
        })
    }

    async fn start(&mut self) -> anyhow::Result<()> {
        info!(
            "Starting Ethereum watcher for contract: {}",
            self.contract_address
        );

        loop {
            if let Err(e) = self.poll_events().await {
                error!("Error polling Ethereum events: {}", e);
            }
            sleep(Duration::from_secs(self.get_polling_interval())).await;
        }
    }

    async fn poll_events(&mut self) -> anyhow::Result<()> {
        let latest_block = self.client.get_block_number().await?;
        let from_block = self.last_block.unwrap_or(self.start_block);

        info!("Latest block: {}", latest_block);

        if from_block >= latest_block {
            info!(
                "Skipping contract {}: last block {} >= latest block {}",
                self.contract_address, from_block, latest_block
            );
            return Ok(());
        }

        info!(
            "Polling contract {} from block {} to {}",
            self.contract_address, from_block, latest_block
        );

        let mut current_block = from_block;
        while current_block < latest_block {
            let next_block = std::cmp::min(current_block + MAX_BLOCK_SPAN, latest_block);

            let filter = Filter::new()
                .from_block(current_block)
                .to_block(next_block)
                .address(alloy::primitives::Address::from_str(
                    &self.contract_address,
                )?);

            let logs = self.client.get_logs(&filter).await?;

            for log in logs {
                // info!("Found log: {:#?}", log);
                self.process_log(log).await?;
            }

            current_block = next_block + 1;
        }

        self.last_block = Some(latest_block);
        Ok(())
    }

    async fn get_block_timestamp(&self, block_number: u64) -> anyhow::Result<u64> {
        let block = self.client.get_block(block_number.into()).await?;
        if block.is_none() {
            return Err(anyhow::anyhow!("Block not found: {}", block_number));
        }
        let block = block.unwrap();
        Ok(block.header.timestamp)
    }

    fn get_polling_interval(&self) -> u64 {
        POLLING_INTERVAL
    }

    async fn process_log(&self, log: Log) -> anyhow::Result<()> {
        // info!("Processing log: {:#?}", log);
        let interface = load_abi(Path::new("./src/abi/escrow_factory.json"))?;

        match decode_log_with_abi(&interface, &log)? {
            Some((event_name, decoded_event)) => {
                info!("Found event: {}", event_name);

                match event_name.as_str() {
                    "SrcEscrowCreated" => {
                        self.handle_src_escrow_created_event(decoded_event, log)
                            .await?;
                    }
                    _ => {
                        info!("Unhandled event type: {}", event_name);
                    }
                }
            }
            None => info!("Could not decode log with provided ABI"),
        }
        Ok(())
    }
}

impl EthereumChain {
    async fn handle_src_escrow_created_event(
        &self,
        decoded_event: DecodedEvent,
        log: Log,
    ) -> anyhow::Result<()> {
        let body = decoded_event.body;

        if body.len() != 3 {
            return Err(anyhow::anyhow!(
                "Expected 3 parameters for SrcEscrowCreated event, got {}",
                body.len()
            ));
        }

        let param_one = match &body[0] {
            DynSolValue::Tuple(tuple_fields) => ParamOneTuple::from_tuple_fields(tuple_fields)?,
            _ => return Err(anyhow::anyhow!("Parameter 0 should be a tuple")),
        };

        let param_two = match &body[1] {
            DynSolValue::Tuple(tuple_fields) => ParamTwoTuple::from_tuple_fields(tuple_fields)?,
            _ => return Err(anyhow::anyhow!("Parameter 1 should be a tuple")),
        };

        let param_three = match &body[2] {
            DynSolValue::Address(addr) => ParamThreeTuple {
                escrow_address: format!("0x{}", hex::encode(addr)),
            },
            _ => return Err(anyhow::anyhow!("Parameter 2 should be an address")),
        };

        let event = SrcEscrowCreatedEvent {
            param_one: param_one.clone(),
            param_two: param_two.clone(),
            param_three: param_three.clone(),
        };

        info!("Parsed SrcEscrowCreated event: {:#?}", event);
        let block_hash = log
            .block_hash
            .ok_or_else(|| anyhow::anyhow!("Missing block_hash"))?;

        self.db
            .handle_escrow_event(
                &format!("{}", param_one.order_hash),
                WatcherEventType::SrcEscrowCreatedEvent,
                &param_three.escrow_address,
                &hex::encode(block_hash),
            )
            .await
            .expect("Failed to Update the database");

        info!(
            "Successfully updated database for order hash: {}",
            param_one.order_hash
        );

        Ok(())
    }
}

fn load_abi(path: &Path) -> anyhow::Result<JsonAbi> {
    let abi_content = fs::read_to_string(path)?;
    let full_json: Value = serde_json::from_str(&abi_content)?;

    let abi_array = full_json
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Missing 'abi' field in contract artifact"))?;

    let json_abi: JsonAbi = serde_json::from_value(Value::Array(abi_array.clone()))?;

    Ok(json_abi)
}

fn decode_log_with_abi(abi: &JsonAbi, log: &Log) -> anyhow::Result<Option<(String, DecodedEvent)>> {
    let topics = log.topics();
    if topics.is_empty() {
        return Err(anyhow::anyhow!("Log has no topics"));
    }

    let selector = topics[0];

    if let Some(event) = abi
        .events()
        .find(|event| B256::from(event.selector()) == selector)
    {
        let log_data = LogData::new_unchecked(topics.to_vec(), log.data().clone().data);
        let decoded = event.decode_log(&log_data)?;
        return Ok(Some((event.name.clone(), decoded)));
    }

    Ok(None)
}
