use std::str::FromStr;

use allfeat_faucet_shared::TransferRequest;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use subxt::utils::AccountId32;
use tokio::spawn;

use crate::{captcha::verify_captcha, chain::transfer_to, Clients};

pub async fn handle_transfer(
    State(state): State<Clients>,
    Json(req): Json<TransferRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    verify_captcha(&req.token)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let account_id = AccountId32::from_str(&req.address).map_err(|_| StatusCode::BAD_REQUEST)?;
    spawn(async move {
        if let Err(e) = transfer_to(account_id, state, req.client_id).await {
            tracing::error!("Transfer failed: {}", e);
        }
    });
    Ok(StatusCode::ACCEPTED)
}
