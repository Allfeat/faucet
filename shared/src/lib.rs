use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize, Serialize)]
/// REST API transfer request params
pub struct TransferRequest {
    pub address: String,
    pub client_id: String,
    pub token: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// The Status communicated through Websocket.
pub enum TransferStatus {
    ApiInit,
    TxSending,
    TxValidated,
    TxBroadcasted,
    TxInBlock { tx_hash: String },

    TxError { error: String },
}

impl Display for TransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: &str = match self {
            TransferStatus::ApiInit => "The backend is preparing the transfer",
            TransferStatus::TxSending => "Transaction is being sent to the network",
            TransferStatus::TxBroadcasted => "Transaction has been propagated to the network",
            TransferStatus::TxValidated => {
                "Transaction has been validated and will be included in the next block"
            }
            TransferStatus::TxInBlock { tx_hash } => {
                &format!("The transfer succeeded with the following hash:\n{tx_hash}",)
            }
            TransferStatus::TxError { error } => &format!("Transfer failed: {error}"),
        };

        write!(f, "{str}")
    }
}
