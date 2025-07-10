use std::time::SystemTime;

use allfeat_faucet_shared::TransferStatus;
use subxt::tx::TxStatus;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, SubstrateConfig};
use subxt_signer::bip39::Mnemonic;
use subxt_signer::sr25519::{dev, Keypair};
use tracing::{debug, error, info};

use crate::websocket::notify_client;
use crate::FaucetState;

#[subxt::subxt(runtime_metadata_path = "melodie_metadata.scale")]
pub mod melodie {}

const DEFAULT_FAUCET_AMOUNT: u128 = 10_000_000_000_000;

fn get_faucet_amount() -> u128 {
    std::env::var("FAUCET_AMOUNT")
        .ok()
        .and_then(|v| v.parse::<u128>().ok())
        .map(|v| v * 10u128.pow(12))
        .unwrap_or(DEFAULT_FAUCET_AMOUNT)
}

fn get_node_url() -> String {
    let endpoint =
        std::env::var("NODE_ENDPOINT_URL").unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
    info!("Using node endpoint: {endpoint}");
    endpoint
}

/// Get the sender account pair from env variable or use the alice account.
fn get_sender_account() -> Keypair {
    let seed = std::env::var("SENDER_SEED").unwrap_or_default();

    Mnemonic::parse(seed)
        .ok()
        .and_then(|m| Keypair::from_phrase(&m, None).ok())
        .unwrap_or_else(dev::alice)
}

pub async fn transfer_to(
    to: AccountId32,
    state: FaucetState,
    client_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let transfer_amount = get_faucet_amount();
    info!(
        "Executing transfer of {} to {}",
        transfer_amount / 10u128.pow(12),
        to.to_string()
    );

    notify_client(&state.ws_clients, &client_id, &TransferStatus::ApiInit).await;

    let api = OnlineClient::<SubstrateConfig>::from_url(get_node_url()).await?;

    notify_client(&state.ws_clients, &client_id, &TransferStatus::TxSending).await;

    let tx = melodie::tx()
        .balances()
        .transfer_allow_death(to.clone().into(), transfer_amount);

    let from = get_sender_account();
    let mut tx_progress = api
        .tx()
        .sign_and_submit_then_watch_default(&tx, &from)
        .await?;

    while let Some(status) = tx_progress.next().await {
        match status {
            Ok(tx_status) => match tx_status {
                TxStatus::Validated => {
                    debug!("Transaction has been validated.");
                    notify_client(&state.ws_clients, &client_id, &TransferStatus::TxValidated)
                        .await;
                }
                TxStatus::Broadcasted => {
                    debug!("Transaction has been broadcasted.");
                    notify_client(
                        &state.ws_clients,
                        &client_id,
                        &TransferStatus::TxBroadcasted,
                    )
                    .await;
                }
                TxStatus::InBestBlock(info) => {
                    debug!(
                        "✅ Transaction included in block. Hash: {}",
                        info.extrinsic_hash()
                    );
                    state
                        .last_claims
                        .write()
                        .await
                        .insert(to.to_string(), SystemTime::now());
                    notify_client(
                        &state.ws_clients,
                        &client_id,
                        &TransferStatus::TxInBlock {
                            tx_hash: info.extrinsic_hash().to_string(),
                        },
                    )
                    .await;
                    return Ok(());
                }
                TxStatus::Error { message }
                | TxStatus::Dropped { message }
                | TxStatus::Invalid { message } => {
                    error!("❌ Transaction failed: {}", message);
                    notify_client(
                        &state.ws_clients,
                        &client_id,
                        &TransferStatus::TxError {
                            error: message.clone(),
                        },
                    )
                    .await;
                    return Err(message.into());
                }
                _ => {
                    debug!("Transaction received status: {:?}", tx_status);
                    return Ok(());
                }
            },
            Err(e) => {
                error!("💥 Error while polling transaction: {}", e);
                return Err(Box::new(e));
            }
        }
    }

    Err("Transaction stream ended unexpectedly".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::ws::Message;
    use std::str::FromStr;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    #[ignore = "E2E chain node test"]
    async fn transfer_to_works() {
        dotenvy::dotenv().ok();
        let bob =
            AccountId32::from_str("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty").unwrap();
        let client_id = "test_client".to_string();

        // Mock client
        let state = FaucetState::default();
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();
        state.ws_clients.write().await.insert(client_id.clone(), tx);

        transfer_to(bob, state, client_id).await.unwrap();

        let received: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
        assert!(!received.is_empty());
    }
}
