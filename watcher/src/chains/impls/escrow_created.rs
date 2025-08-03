use alloy::{
    dyn_abi::{DecodedEvent, DynSolValue},
    hex,
    primitives::{Address, U256},
    rpc::types::Log,
};
use tracing::info;

use crate::{chains::ethereum::EthereumChain, types::WatcherEventType};

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

#[derive(Debug, Clone)]
pub struct DstEscrowCreatedEvent {
    pub escrow_address: String,
    pub hashlock: String,
    pub taker_address: String,
    pub order_hash: String,
}

impl EthereumChain {
    pub async fn handle_src_escrow_created_event(
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
                &param_one.order_hash.to_string(),
                WatcherEventType::SrcEscrowCreatedEvent,
                &param_three.escrow_address,
                &hex::encode(block_hash),
                log,
            )
            .await
            .expect("Failed to Update the database");

        info!(
            "Successfully updated database for order hash: {}",
            event.param_one.order_hash
        );

        Ok(())
    }

    pub async fn handle_dst_escrow_created_event(
        &self,
        decoded_event: DecodedEvent,
        log: Log,
    ) -> anyhow::Result<()> {
        let body = decoded_event.body;

        if body.len() != 4 {
            return Err(anyhow::anyhow!(
                "Expected 4 parameters for DstEscrowCreated event, got {}",
                body.len()
            ));
        }

        let event = DstEscrowCreatedEvent::from_decoded_body(&body)?;

        info!("Parsed DstEscrowCreated event: {:#?}", event);

        let block_hash = log
            .block_hash
            .ok_or_else(|| anyhow::anyhow!("Missing block_hash"))?;

        self.db
            .handle_escrow_event(
                &event.order_hash,
                WatcherEventType::DstEscrowCreatedEvent,
                &event.escrow_address,
                &hex::encode(block_hash),
                log,
            )
            .await
            .expect("Failed to Update the database");

        info!(
            "Successfully updated database for order_hash: {}",
            event.order_hash
        );

        Ok(())
    }
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

        let maker_address = Self::extract_address_from_uint(&fields[2], "Field 2")?;
        let taker_address = Self::extract_address_from_uint(&fields[3], "Field 3")?;
        let token_address = Self::extract_address_from_uint(&fields[4], "Field 4")?;

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

    fn extract_address_from_uint(value: &DynSolValue, field_name: &str) -> anyhow::Result<String> {
        match value {
            DynSolValue::Uint(value, _) => {
                let addr_bytes = value.to_be_bytes::<32>();
                let addr = Address::from_slice(&addr_bytes[12..]);
                Ok(format!("0x{}", hex::encode(addr)))
            }
            _ => Err(anyhow::anyhow!("{} should be Uint (address)", field_name)),
        }
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

        let maker_address = Self::extract_address_from_uint(&fields[0], "Field 0")?;

        let taker_amount = match &fields[1] {
            DynSolValue::Uint(value, _) => *value,
            _ => return Err(anyhow::anyhow!("Field 1 should be Uint")),
        };

        let destination_token_address = Self::extract_address_from_uint(&fields[2], "Field 2")?;

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

    fn extract_address_from_uint(value: &DynSolValue, field_name: &str) -> anyhow::Result<String> {
        match value {
            DynSolValue::Uint(value, _) => {
                let addr_bytes = value.to_be_bytes::<32>();
                let addr = Address::from_slice(&addr_bytes[12..]);
                Ok(format!("0x{}", hex::encode(addr)))
            }
            _ => Err(anyhow::anyhow!("{} should be Uint (address)", field_name)),
        }
    }
}

impl DstEscrowCreatedEvent {
    fn from_decoded_body(body: &[DynSolValue]) -> anyhow::Result<Self> {
        if body.len() != 4 {
            return Err(anyhow::anyhow!(
                "Expected 4 parameters for DstEscrowCreated event, got {}",
                body.len()
            ));
        }

        let escrow_address = match &body[0] {
            DynSolValue::Address(addr) => format!("0x{}", hex::encode(addr)),
            _ => return Err(anyhow::anyhow!("Parameter 0 should be an address")),
        };

        let hashlock = match &body[1] {
            DynSolValue::FixedBytes(bytes, _) => hex::encode(bytes),
            _ => return Err(anyhow::anyhow!("Parameter 1 should be fixed bytes")),
        };

        let taker_address = match &body[2] {
            DynSolValue::Address(addr) => format!("0x{}", hex::encode(addr)),
            DynSolValue::Uint(value, _) => {
                let addr_bytes = value.to_be_bytes::<32>();
                let addr = Address::from_slice(&addr_bytes[12..]);
                format!("0x{}", hex::encode(addr))
            }
            _ => return Err(anyhow::anyhow!("Parameter 2 should be an address")),
        };

        let order_hash = match &body[3] {
            DynSolValue::FixedBytes(bytes, _) => hex::encode(bytes),
            _ => return Err(anyhow::anyhow!("Parameter 3 should be in bytes")),
        };

        Ok(DstEscrowCreatedEvent {
            escrow_address,
            hashlock,
            taker_address,
            order_hash,
        })
    }
}
