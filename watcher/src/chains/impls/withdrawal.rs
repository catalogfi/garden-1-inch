use alloy::{
    dyn_abi::{DecodedEvent, DynSolValue},
    hex,
    rpc::types::Log,
};
use tracing::info;

use crate::{chains::ethereum::EthereumChain, types::SecretEntry};

#[derive(Debug, Clone)]
pub struct ParamOne {
    pub secret: String,
}

#[derive(Debug, Clone)]
pub struct WithdrawalEvent {
    pub param_one: ParamOne,
}

impl EthereumChain {
    pub async fn handle_withdrawn_event(
        &self,
        decoded_event: DecodedEvent,
        _log: Log,
        order_hash: &str,
    ) -> anyhow::Result<()> {
        let body = decoded_event.body;
        if body.len() != 1 {
            return Err(anyhow::anyhow!(
                "Expected 1 parameter for Withdrawal event, got {}",
                body.len()
            ));
        }

        let secret_bytes = match &body[0] {
            DynSolValue::FixedBytes(secret, _) => secret,
            _ => return Err(anyhow::anyhow!("Parameter 0 should be a FixedBytes")),
        };

        let secret_hex = hex::encode(secret_bytes);

        let event = WithdrawalEvent {
            param_one: ParamOne {
                secret: secret_hex.clone(),
            },
        };
        info!("Parsed Withdraw event: {:#?}", event);

        let secret_entry = SecretEntry {
            index: 0,
            secret: Some(event.param_one.secret),
            secret_hash: secret_hex,
        };

        let secrets_vec = vec![secret_entry];
        let secrets_value = serde_json::to_value(&secrets_vec)
            .map_err(|e| anyhow::anyhow!("Failed to serialize secrets: {}", e))?;

        self.db
            .update_secrets(order_hash, &secrets_value)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to update secrets: {}", e))?;

        Ok(())
    }
}
