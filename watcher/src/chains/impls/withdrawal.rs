use crate::chains::ethereum::EthereumChain;
use alloy::{
    dyn_abi::{DecodedEvent, DynSolValue},
    hex::{self},
    rpc::types::Log,
};
use tracing::info;

#[derive(Debug, Clone)]
pub struct ParamOne {
    pub secret: String,
    pub order_hash: String,
}

#[derive(Debug, Clone)]
pub struct WithdrawalEvent {
    pub param_one: ParamOne,
}
impl EthereumChain {
    pub async fn handle_withdrawn_event(
        &self,
        decoded_event: DecodedEvent,
        log: Log,
    ) -> anyhow::Result<()> {
        let body = decoded_event.body;
        if body.len() != 2 {
            return Err(anyhow::anyhow!(
                "Expected 2 parameters for Withdrawal event, got {}",
                body.len()
            ));
        }

        // let secret_bytes = match &body[0] {
        //     DynSolValue::FixedBytes(secret, _) => secret,
        //     _ => return Err(anyhow::anyhow!("Parameter 0 should be a FixedBytes")),
        // };

        let event_order_hash = match &body[1] {
            DynSolValue::FixedBytes(order_hash_bytes, _) => hex::encode(order_hash_bytes),
            _ => return Err(anyhow::anyhow!("Parameter 1 should be FixedBytes")),
        };

        let escrow_address = format!("0x{}", hex::encode(log.address()));

        let status = self
            .db
            .determine_withdrawal_status(&event_order_hash, &escrow_address)
            .await?;

        self.db
            .update_order_status(&event_order_hash, status)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to update status: {}", e))?;

        info!(
            "Successfully processed withdrawal for order hash: {}",
            event_order_hash
        );

        Ok(())
    }
}
